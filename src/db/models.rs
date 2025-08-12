use crate::{
    db::schema::erc20_transfer_events, event::EventMessage, transfer_event::TransferEvent,
};
use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable, prelude::Insertable};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = erc20_transfer_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Erc20TranferEvents {
    pub id: i32,
    pub from: String,
    pub to: String,
    pub value: String,
    pub block_number: String,
    pub tx_hash: String,
    pub log_index: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = erc20_transfer_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewErc20TransferEvent {
    pub from: String,
    pub to: String,
    pub value: String,
    pub block_number: String,
    pub tx_hash: String,
    pub log_index: String,
}

impl From<EventMessage<TransferEvent>> for NewErc20TransferEvent {
    fn from(value: EventMessage<TransferEvent>) -> Self {
        Self {
            // ethers fmt::Display impl for H{160,256} types returns shortened version therefore we need to format in debug
            from: format!("{:?}", value.event.from),
            to: format!("{:?}", value.event.to),
            value: value.event.value.to_string(),
            block_number: value.block_number.to_string(),
            tx_hash: format!("{:?}", value.tx_hash),
            log_index: value.log_index.to_string(),
        }
    }
}
