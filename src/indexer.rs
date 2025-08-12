use crate::event::TopicFilters;
use crate::log_consumer::ConsumeEvent;
use crate::{
    db::repository::EventRepository, errors::IndexerResult, event::EventIndexingInfo,
    log_consumer::TypedLogConsumer,
};
use ethers::middleware::Middleware;
use ethers::{
    providers::{JsonRpcClient, Provider, PubsubClient},
    types::{Filter, FilterBlockOption},
};
use futures::StreamExt;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

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
        T: DeserializeOwned + Debug + Send + Sync,
        R: EventRepository<EventType = T> + Send + Sync,
    {
        let filter: Filter = event_indexing_info.into();

        let mut stream = self.provider.subscribe_logs(&filter).await?;
        while let Some(log) = stream.next().await {
            log_consumer.consume_event(log).await?;
        }
        Ok(())
    }
}

impl From<EventIndexingInfo> for Filter {
    fn from(value: EventIndexingInfo) -> Self {
        let block_filter: FilterBlockOption = value.block_filter.clone().into();

        let filter = Filter::new()
            .address(value.contract)
            .select(block_filter)
            .topic0(value.event.signature());

        if let Some(filters) = value.filters {
            let TopicFilters {
                topic1,
                topic2,
                topic3,
            } = filters;

            let filter = if let Some(topic1) = topic1 {
                filter.topic1(topic1)
            } else {
                filter
            };

            let filter = if let Some(topic2) = topic2 {
                filter.topic2(topic2)
            } else {
                filter
            };

            if let Some(topic3) = topic3 {
                filter.topic3(topic3)
            } else {
                filter
            }
        } else {
            filter
        }
    }
}
