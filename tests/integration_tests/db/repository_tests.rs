use std::str::FromStr;

use diesel::{RunQueryDsl, SqliteConnection, r2d2::ConnectionManager};
use ethers::types::{H160, H256, U64, U256};
use logger_playground::{
    db::{
        models::{Erc20TranferEvent, NewErc20TransferEvent},
        repository::EventRepository,
        transfer_repository::ERC20TransferRepo,
    },
    event::EventMessage,
    transfer_event::TransferEvent,
};
use r2d2::Pool;

use crate::common::{
    fill_tables::{BLOCK_NUMBER, FROM, LOG_INDEX, TO, TX_HASH, VALUE},
    setup_db,
};

fn create_erc20_repo(pool: Pool<ConnectionManager<SqliteConnection>>) -> ERC20TransferRepo {
    ERC20TransferRepo::new(pool)
}

fn event_message() -> EventMessage<TransferEvent> {
    EventMessage {
        block_number: U64::from(BLOCK_NUMBER),
        tx_hash: H256::from_str(TX_HASH).expect("expected tx_hash"),
        log_index: U256::from(LOG_INDEX),
        event: TransferEvent {
            from: H160::from_str(FROM).expect("expected from"),
            to: H160::from_str(TO).expect("expected to"),
            value: U256::from(VALUE),
        },
    }
}

#[tokio::test]
async fn test_store_event() {
    let pool = setup_db();
    let repo = create_erc20_repo(pool.clone());
    let event = event_message();

    let store_res = repo.store_event(event.clone()).await;

    assert!(store_res.is_ok());

    use logger_playground::db::schema::erc20_transfer_events;

    let res = erc20_transfer_events::table
        .load::<Erc20TranferEvent>(&mut pool.get().expect("expected conn"));

    assert!(res.is_ok());

    let events = res.expect("expected events");
    assert!(events.len().eq(&1));

    let first = events.first().expect("expected first element").to_owned();
    let new_erc20_event = NewErc20TransferEvent::from(event);

    assert_eq!(first.block_number, new_erc20_event.block_number);
    assert_eq!(first.tx_hash, new_erc20_event.tx_hash);
    assert_eq!(first.log_index, new_erc20_event.log_index);
    assert_eq!(first.from, new_erc20_event.from);
    assert_eq!(first.to, new_erc20_event.to);
    assert_eq!(
        first.value.parse::<f64>().expect("expected value"),
        new_erc20_event
            .value
            .parse::<f64>()
            .expect("expected value")
    );
}
