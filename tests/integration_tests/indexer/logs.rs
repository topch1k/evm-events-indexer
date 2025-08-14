use ethers::types::{Bytes, H256, U64};
use ethers::types::{Log, U256};
use logger_playground::event::EventIndexingInfo;

pub fn sample_log() -> Log {
    Log {
        address: "dac17f958d2ee523a2206206994597c13d831ec7".parse().unwrap(),
        topics: vec![
            H256::from_slice(
                &hex::decode("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
                    .unwrap(),
            ),
            H256::from_slice(
                &hex::decode("00000000000000000000000000c600b30fb0400701010f4b080409018b9006e0")
                    .unwrap(),
            ),
            H256::from_slice(
                &hex::decode("00000000000000000000000066a9893cc07d91d95644aedd05d03f95e1dba8af")
                    .unwrap(),
            ),
        ],
        data: Bytes::from(
            hex::decode("00000000000000000000000000000000000000000000000000000048e4699f0c")
                .unwrap(),
        ),
        block_hash: Some(H256::from_slice(
            &hex::decode("108bc30acae1d39d94defc1060ea70632df87ea744c31a8f6b54e518e98dd3e1")
                .unwrap(),
        )),
        block_number: Some(U64::from(23138511)),
        transaction_hash: Some(H256::from_slice(
            &hex::decode("b9135bd4b40082d3dd028e685e306aa4991f7a3f0c263628a62c237d0dd17b5c")
                .unwrap(),
        )),
        transaction_index: Some(U64::from(136)),
        log_index: Some(U256::from(348)),
        transaction_log_index: None,
        log_type: None,
        removed: Some(false),
    }
}

pub fn sample_event_indexing_info() -> EventIndexingInfo {
    EventIndexingInfo {
        event: ethers::abi::Event {
            name: "Transfer".to_string(),
            inputs: vec![
                ethers::abi::EventParam {
                    name: "from".to_string(),
                    kind: ethers::abi::ParamType::Address,
                    indexed: true,
                },
                ethers::abi::EventParam {
                    name: "to".to_string(),
                    kind: ethers::abi::ParamType::Address,
                    indexed: true,
                },
                ethers::abi::EventParam {
                    name: "value".to_string(),
                    kind: ethers::abi::ParamType::Uint(256),
                    indexed: false,
                },
            ],
            anonymous: false,
        },
        contract: "0xdAC17F958D2ee523a2206206994597C13D831ec7"
            .parse()
            .unwrap(),
        block_filter: logger_playground::event::BlockFilter::Range {
            from_block: ethers::types::BlockNumber::Latest,
            to_block: None,
        },
        filters: None,
    }
}
