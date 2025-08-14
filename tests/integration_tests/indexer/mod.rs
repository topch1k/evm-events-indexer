pub mod logs;

use ethers::prelude::*;
use ethers::types::Log;
use ethers::types::U256;
use futures::Stream;
use logger_playground::errors::IndexerResult;
use logger_playground::indexer::LogIndexer;
use logger_playground::interfaces::consume_event::ConsumeEvent;
use logger_playground::transfer_event::TransferEvent;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;
use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

use crate::indexer::logs::sample_event_indexing_info;
use crate::indexer::logs::sample_log;

pub struct MockConsumer {
    pub expected_log: Log,
}
impl MockConsumer {
    pub fn new(expected_log: Log) -> Self {
        Self { expected_log }
    }
}

#[async_trait::async_trait]
impl ConsumeEvent<TransferEvent> for MockConsumer {
    async fn consume_event(&self, log: Log) -> IndexerResult<()> {
        assert_eq!(self.expected_log, log);
        Ok(())
    }
}

#[derive(Debug)]
pub struct CustomMockProvider<T> {
    data: VecDeque<T>,
    responses: Arc<Mutex<VecDeque<MockResponse>>>,
}

impl<T> CustomMockProvider<T> {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            responses: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

impl<T> Default for CustomMockProvider<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D> CustomMockProvider<D> {
    pub fn push<T: Serialize + Send + Sync, K: Borrow<T>>(&self, data: K) -> Result<(), MockError> {
        let value = serde_json::to_value(data.borrow())?;
        self.responses
            .lock()
            .unwrap()
            .push_back(MockResponse::Value(value));
        Ok(())
    }
}

#[async_trait::async_trait]
impl<D> JsonRpcClient for CustomMockProvider<D>
where
    D: Debug + Send + Sync,
{
    type Error = MockError;

    async fn request<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        _params: T,
    ) -> Result<R, MockError> {
        match method {
            "eth_subscribe" => serde_json::from_value::<R>(serde_json::to_value(U256::from(1))?)
                .map_err(Into::into),
            "eth_getLogs" => {
                let mut data = self.responses.lock().unwrap();
                let element = data.pop_back().ok_or(MockError::EmptyResponses)?;
                match element {
                    MockResponse::Value(value) => {
                        let log = serde_json::from_value::<Log>(value)
                            .expect("expected from_value to Log");
                        let vec = vec![log];
                        let value = serde_json::to_value(vec).expect("expected value vec");

                        let res: R = serde_json::from_value(value)
                            .inspect_err(|e| println!("From value error : {e:?}"))?;

                        Ok(res)
                    }
                    MockResponse::Error(error) => Err(MockError::JsonRpcError(error)),
                }
            }
            other => {
                println!("Other method: {other}");
                Err(MockError::EmptyResponses)
            }
        }
    }
}

impl<D> PubsubClient for CustomMockProvider<D>
where
    D: Debug + Unpin + Send + Sync + Clone + Serialize,
{
    type NotificationStream = MockStream<D>;

    fn subscribe<T: Into<U256>>(&self, _id: T) -> Result<Self::NotificationStream, Self::Error> {
        let stream = MockStream::new(self);
        Ok(stream)
    }

    fn unsubscribe<T: Into<U256>>(&self, _id: T) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct MockStream<T> {
    data: VecDeque<T>,
}

impl<T: Clone> MockStream<T> {
    pub fn new(provider: &CustomMockProvider<T>) -> Self {
        Self {
            data: provider.data.clone(),
        }
    }
}

impl<T: Serialize + Unpin + Debug> Stream for MockStream<T> {
    type Item = Box<RawValue>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let val = this.data.pop_back();
        let raw_value = match val {
            Some(v) => {
                let v = serde_json::to_string(&v).expect("expected to_string");
                RawValue::from_string(v).ok()
            }
            None => None,
        };

        std::task::Poll::Ready(raw_value)
    }
}

#[tokio::test]
async fn test_log_indexer() {
    let consumer = MockConsumer::new(sample_log());
    let custom_provider = CustomMockProvider::<Log>::new();

    custom_provider
        .push::<_, Log>(sample_log())
        .expect("expected push data");

    let provider = Provider::new(custom_provider);

    let indexer = LogIndexer::new(provider);
    let _ = indexer
        .run(sample_event_indexing_info(), &consumer)
        .await
        .inspect_err(|e| println!("Error during listening : {e:?}"));
}
