use diesel::{RunQueryDsl, SqliteConnection, dsl::insert_into, r2d2::ConnectionManager};
use r2d2::Pool;
// use r2d2::Pool;
// use r2d2_sqlite::SqliteConnectionManager;
// use r2d2_sqlite::SqliteConnectionManager;

use crate::{
    db::{
        models::NewErc20TransferEvent, repository::EventRepository, schema::erc20_transfer_events,
    },
    errors::Errors,
    event::EventMessage,
    transfer_event::TransferEvent,
};

pub struct ERC20TransferRepo {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl ERC20TransferRepo {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl EventRepository for ERC20TransferRepo {
    type Err = Errors;
    type EventType = TransferEvent;
    async fn store_event(&self, event: EventMessage<TransferEvent>) -> Result<(), Self::Err> {
        log::trace!("Storing event : {event:?}");

        let new_event: NewErc20TransferEvent = event.into();

        let mut conn = self.pool.get()?;

        let res = insert_into(erc20_transfer_events::table)
            .values(&new_event)
            .on_conflict_do_nothing()
            .execute(&mut conn)?;

        log::debug!("Inserting res : {res}",);

        Ok(())
    }

    async fn get_event(&self, _some_event_feature: ()) {
        log::debug!("Getting event");

        todo!();
    }
}
