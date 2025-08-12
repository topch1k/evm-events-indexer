use clap::Subcommand;
use ethers::types::H256;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Start,
    ListBy {
        #[clap(subcommand)]
        filter_by: FilterBy,
    },
}
#[derive(Debug, Clone, Subcommand)]
pub enum FilterBy {
    Ids { ids: Vec<i32> },
    From { from: String },
    To { to: String },
    TxHash { hash: H256 },
    BlockNumber { block_number: u64 },
}
