use r2d2_sqlite::SqliteConnectionManager;

use crate::{db::repository::EventRepository, event::EventMessage, transfer_event::TransferEvent};

pub struct ERC20TransferRepo {
    _pool: r2d2::Pool<SqliteConnectionManager>,
}

impl ERC20TransferRepo {
    pub fn new(pool: r2d2::Pool<SqliteConnectionManager>) -> Self {
        Self { _pool: pool }
    }
}

#[async_trait::async_trait]
impl EventRepository for ERC20TransferRepo {
    type Err = ();
    type EventType = TransferEvent;
    async fn store_event(&self, event: EventMessage<TransferEvent>) -> Result<(), Self::Err> {
        log::debug!("Storing event : {event:?}");

        let EventMessage {
            block_number,
            tx_hash,
            log_index,
            event: TransferEvent { from, to, value },
        } = event;

        let conn = self._pool.get().unwrap(); //TODO:
        let res = conn
            .execute(
                "INSERT INTO erc20_transfer_events(id, \"from\", \"to\", \"value\", block_number, tx_hash, log_index) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7 ) ON CONFLICT DO NOTHING",
                (
                    None::<u64>,
                    from.to_string(),
                    to.to_string(),
                    value.to_string(),
                    block_number.to_string(),
                    tx_hash.to_string(),
                    log_index.to_string(),
                )
            )
            .unwrap(); //TODO:

        log::debug!("Inserting res : {res}",);

        Ok(())
    }

    async fn get_event(&self, _some_event_feature: ()) {
        log::debug!("Getting event");

        todo!();
    }
}
