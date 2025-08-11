use std::fmt::Debug;

use ethers::{
    providers::{JsonRpcClient, Provider, PubsubClient},
    types::{Filter, FilterBlockOption},
};
use serde::de::DeserializeOwned;

use crate::{
    db::repository::EventRepository, errors::IndexerResult, event::EventIndexingInfo,
    log_consumer::TypedLogConsumer,
};
use ethers::middleware::Middleware;
use futures::StreamExt;

pub struct LogIndexer<P> {
    pub provider: Provider<P>,
    pub event: EventIndexingInfo,
}

impl<P> LogIndexer<P> {
    pub fn new(provider: Provider<P>, event: EventIndexingInfo) -> Self {
        Self { provider, event }
    }
}

impl<P> LogIndexer<P>
where
    P: JsonRpcClient + PubsubClient,
{
    pub async fn run<T, R>(
        self,
        event_indexing_info: EventIndexingInfo,
        log_consumer: &TypedLogConsumer<T, R>,
    ) -> IndexerResult<()>
    where
        T: DeserializeOwned + Debug,
        R: EventRepository<EventType = T>,
    {
        let filter: Filter = event_indexing_info.into();

        let mut stream = self.provider.subscribe_logs(&filter).await?;
        while let Some(log) = stream.next().await {
            log::debug!("Log from indexer : {log:?}");
            log_consumer.consume_events(log).await?;
        }
        Ok(())
    }
}

impl From<EventIndexingInfo> for Filter {
    fn from(value: EventIndexingInfo) -> Self {
        let block_filter = match value.to_block {
            Some(to_block) => FilterBlockOption::Range {
                from_block: Some(value.from_block),
                to_block: Some(to_block),
            },
            None => todo!(),
        };

        Filter::new()
            .address(value.contract)
            .select(block_filter)
            .topic0(value.event.signature())
    }
}
