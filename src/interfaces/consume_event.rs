use crate::errors::IndexerResult;
use ethers::types::Log;

#[async_trait::async_trait]
pub trait ConsumeEvent<T> {
    async fn consume_event(&self, log: Log) -> IndexerResult<()>;
}
