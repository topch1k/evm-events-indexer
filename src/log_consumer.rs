use std::{fmt::Debug, marker::PhantomData};

use crate::{
    db::repository::EventRepository,
    errors::IndexerResult,
    event::{EventMessage, RawEventMessage},
};
use ethers::{
    abi::{Event, RawLog},
    types::Log,
};
use serde::de::DeserializeOwned;

pub struct TypedLogConsumer<T, R> {
    pub event: Event,
    pub repo: R,
    _phantom: PhantomData<T>,
}

impl<T, R> TypedLogConsumer<T, R> {
    pub fn new(event: Event, repository: R) -> Self {
        Self {
            event,
            repo: repository,
            _phantom: PhantomData,
        }
    }
}

impl<T, R: EventRepository<EventType = T>> TypedLogConsumer<T, R>
where
    T: DeserializeOwned + Debug,
{
    pub async fn consume_events(&self, log: Log) -> IndexerResult<()> {
        let raw_log: RawLog = (log.topics, log.data.to_vec()).into();
        let parsed_log = self.event.parse_log(raw_log)?;
        let raw_event_message = RawEventMessage::try_from(parsed_log)?;

        let log_json = serde_json::to_value(&raw_event_message)?;
        let value: T = serde_json::from_value(log_json)?;

        let event_msg = EventMessage::<T> {
            block_number: log.block_number.unwrap_or_default(),
            tx_hash: log.transaction_hash.unwrap_or_default(),
            log_index: log.log_index.unwrap_or_default(),
            event: value,
        };

        let _ = self
            .repo
            .store_event(event_msg)
            .await
            .inspect_err(|e| log::warn!("Storing event error : {e:?}")); //TODO:

        Ok(())
    }
}
