use crate::event::EventMessage;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait EventRepository {
    type Err: Debug;
    type EventType;
    type Filter;
    type EventEntity;
    type Page;
    async fn store_event(&self, event: EventMessage<Self::EventType>) -> Result<(), Self::Err>;

    async fn get_events_by(
        &self,
        query_filter: Self::Filter,
        page: Self::Page,
    ) -> Result<Vec<Self::EventEntity>, Self::Err>;
}
