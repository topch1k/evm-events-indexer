use crate::{
    db::schema::erc20_transfer_events, event::EventMessage, transfer_event::TransferEvent,
};
use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable, prelude::Insertable};

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = erc20_transfer_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Erc20TranferEvent {
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
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::event::EventMessage;
    use crate::transfer_event::TransferEvent;
    use ethers::types::{H160, H256, U64, U256};

    const FROM: &str = "0xdac17f958d2ee523a2206206994597c13d831ec7";
    const TO: &str = "110ba8f305a5ae6815b4fa123a61d94d26b4e1f7";
    const VALUE: u64 = 1000;
    const BLOCK_NUMBER: u64 = 1234;
    const TX_HASH: &str = "0x378108375f820b65276b6db3a180faa0b71a28b8aa24234dd7c143db149ea3b9";
    const LOG_INDEX: u64 = 7;

    #[test]
    fn test_from_event_message_for_new_erc20_transfer_event() {
        let from = H160::from_str(FROM).expect("expected from");
        let to = H160::from_str(TO).expect("expected TO");
        let value = U256::from(VALUE);
        let block_number = U64::from(BLOCK_NUMBER);
        let tx_hash = H256::from_str(TX_HASH).expect("expected tx_hash");
        let log_index = U256::from(LOG_INDEX);

        let transfer_event = TransferEvent { from, to, value };

        let event_message = EventMessage {
            event: transfer_event,
            block_number,
            tx_hash,
            log_index,
        };

        let new_event: NewErc20TransferEvent = event_message.into();

        assert_eq!(new_event.from, format!("{:?}", from));
        assert_eq!(new_event.to, format!("{:?}", to));
        assert_eq!(new_event.value, value.to_string());
        assert_eq!(new_event.block_number, block_number.to_string());
        assert_eq!(new_event.tx_hash, format!("{:?}", tx_hash));
        assert_eq!(new_event.log_index, log_index.to_string());
    }
}
