use crate::errors::{Errors, IndexerResult};
use crate::interfaces::consume_event::ConsumeEvent;
use ethers::middleware::Middleware;
use ethers::providers::Ws;
use ethers::{
    providers::{JsonRpcClient, Provider, PubsubClient},
    types::Filter,
};
use futures::StreamExt;

pub type WsLogIndexer = LogIndexer<Provider<Ws>>;

pub struct LogIndexer<M> {
    pub provider: M,
}

impl<M> LogIndexer<M>
where
    M: Middleware,
{
    pub fn new(provider: M) -> Self {
        Self { provider }
    }
}

impl<M> LogIndexer<M>
where
    M: Middleware<Provider: JsonRpcClient + PubsubClient, Error: Into<Errors>>,
{
    pub async fn run<T, E, C>(self, event_indexing_info: E, log_consumer: &C) -> IndexerResult<()>
    where
        E: Into<Filter>,
        C: ConsumeEvent<T>,
    {
        let filter: Filter = event_indexing_info.into();

        let mut stream = self
            .provider
            .subscribe_logs(&filter)
            .await
            .map_err(Into::<Errors>::into)?;
        while let Some(log) = stream.next().await {
            log::info!("{log:?}");
            log_consumer.consume_event(log).await?;
        }
        Ok(())
    }
}
