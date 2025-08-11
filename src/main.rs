use crate::{
    db::{init::init_db, transfer_repository::ERC20TransferRepo},
    errors::IndexerResult,
    indexer::LogIndexer,
    log_consumer::TypedLogConsumer,
    transfer_event::TransferEvent,
};
use ethers::providers::{Provider, Ws};
use std::path::PathBuf;

pub mod config;
pub mod db;
pub mod errors;
pub mod event;
pub mod indexer;
pub mod log_consumer;
pub mod transfer_event;

#[tokio::main]
async fn main() -> IndexerResult<()> {
    env_logger::init();

    let args = config::parse();
    let conf = config::load(PathBuf::from(args.config_path))?;

    let pool = init_db(&conf.db_path).unwrap(); //TODO:
    let repo = ERC20TransferRepo::new(pool);

    let consumer = TypedLogConsumer::<TransferEvent, _>::new(conf.event_info.event.clone(), repo);

    let provider = Provider::<Ws>::connect(conf.node_url).await?;
    let indexer = LogIndexer::new(provider, conf.event_info.clone());
    let _ = indexer.run(conf.event_info, &consumer).await?;

    Ok(())
}
