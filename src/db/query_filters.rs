use ethers::types::H256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
pub enum FilterBy {
    Ids(Vec<i32>),
    From(String),
    To(String),
    TxHash(H256),
    BlockNumber(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl Page {
    pub fn new(offset: Option<i64>, limit: Option<i64>) -> Self {
        Self { offset, limit }
    }
}

impl From<crate::cli::commands::FilterBy> for FilterBy {
    fn from(value: crate::cli::commands::FilterBy) -> Self {
        match value {
            crate::cli::commands::FilterBy::Ids { ids } => Self::Ids(ids),
            crate::cli::commands::FilterBy::From { from } => Self::From(from),
            crate::cli::commands::FilterBy::To { to } => Self::To(to),
            crate::cli::commands::FilterBy::TxHash { hash } => Self::TxHash(hash),
            crate::cli::commands::FilterBy::BlockNumber { block_number } => {
                Self::BlockNumber(block_number)
            }
        }
    }
}
