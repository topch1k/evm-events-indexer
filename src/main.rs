use crate::{
    db::{
        init::init_db, query_filters::Page, repository::EventRepository,
        transfer_repository::ERC20TransferRepo,
    },
    errors::IndexerResult,
    indexer::WsLogIndexer,
    log_consumer::EventsDbStorage,
    transfer_event::TransferEvent,
};
use ethers::providers::{Provider, Ws};
use std::path::PathBuf;

pub mod cli;
pub mod db;
pub mod errors;
pub mod event;
pub mod indexer;
pub mod log_consumer;
pub mod transfer_event;

#[tokio::main]
async fn main() -> IndexerResult<()> {
    env_logger::init();

    let args = cli::config::parse();
    let conf = cli::config::load(PathBuf::from(args.config_path))?;

    let pool = init_db(&conf.db_path)?;
    let repo = ERC20TransferRepo::new(pool);

    match args.command {
        cli::commands::Commands::Start => {
            let consumer =
                EventsDbStorage::<_, TransferEvent>::new(conf.event_info.event.clone(), repo);

            let provider = Provider::<Ws>::connect(conf.node_url).await?;
            let indexer = WsLogIndexer::new(provider);
            indexer.run(conf.event_info, &consumer).await?;
        }
        cli::commands::Commands::ListBy { filter_by } => {
            let events = repo
                .get_events_by(filter_by.into(), Page::new(args.offset, args.limit))
                .await?;

            log::info!("{events:?}"); //TODO:
        }
    }

    Ok(())
}
