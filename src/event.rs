use std::collections::HashMap;

use ethers::{
    abi::{Event, Token},
    types::{Address, BlockNumber, H256, U256, U64},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::IndexerResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EventIndexingInfo {
    pub event: Event,
    pub contract: Address,
    pub from_block: BlockNumber,
    pub to_block: Option<BlockNumber>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEventMessage {
    #[serde(flatten)]
    pub values: HashMap<String, Value>,
}

impl TryFrom<ethers::abi::Log> for RawEventMessage {
    type Error = crate::errors::Errors;
    fn try_from(value: ethers::abi::Log) -> Result<Self, Self::Error> {
        let mut values: HashMap<String, Value> = HashMap::with_capacity(value.params.len());
        for param in value.params {
            let name = param.name;
            let value = eth_token_to_json_value(param.value)?;
            values.insert(name, value);
        }

        Ok(RawEventMessage { values })
    }
}

fn eth_token_to_json_value(token: Token) -> IndexerResult<Value> {
    let value = match token {
        Token::Address(h160) => serde_json::to_value(h160)?,
        Token::FixedBytes(items) => serde_json::to_value(format!("0x{}", hex::encode(items)))?,
        Token::Bytes(items) => serde_json::to_value(format!("0x{}", hex::encode(items)))?,
        Token::Int(u256) => serde_json::to_value(u256)?,
        Token::Uint(u256) => serde_json::to_value(u256)?,
        Token::Bool(v) => serde_json::to_value(v)?,
        Token::String(v) => serde_json::to_value(v)?,
        Token::FixedArray(tokens) => {
            let values = tokens
                .into_iter()
                .map(eth_token_to_json_value)
                .collect::<IndexerResult<Vec<_>>>()?;
            serde_json::Value::Array(values)
        }
        Token::Array(tokens) => {
            let values = tokens
                .into_iter()
                .map(eth_token_to_json_value)
                .collect::<IndexerResult<Vec<_>>>()?;
            serde_json::Value::Array(values)
        }
        Token::Tuple(tokens) => {
            let values = tokens
                .into_iter()
                .map(eth_token_to_json_value)
                .collect::<IndexerResult<Vec<_>>>()?;
            serde_json::Value::Array(values)
        }
    };
    Ok(value)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventMessage<T> {
    pub block_number: U64,
    pub tx_hash: H256,
    pub log_index: U256,
    pub event: T,
}
