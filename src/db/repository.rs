use std::fmt::Debug;

use crate::event::EventMessage;

#[async_trait::async_trait]
pub trait EventRepository {
    type Err: Debug;
    type EventType;
    async fn store_event(&self, event: EventMessage<Self::EventType>) -> Result<(), Self::Err>;

    async fn get_event(&self, some_event_feature: ());
}
