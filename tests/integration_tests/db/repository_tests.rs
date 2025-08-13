use std::str::FromStr;

use diesel::{RunQueryDsl, SqliteConnection, r2d2::ConnectionManager};
use ethers::types::{H160, H256, U64, U256};
use logger_playground::{
    db::{
        models::{Erc20TranferEvent, NewErc20TransferEvent},
        query_filters::{FilterBy, Page},
        repository::EventRepository,
        transfer_repository::ERC20TransferRepo,
    },
    event::EventMessage,
    transfer_event::TransferEvent,
};
use r2d2::Pool;

use crate::common::{
    fill_tables::{BLOCK_NUMBER, FROM, LOG_INDEX, TO, TX_HASH, VALUE, fill_db_with_test_data},
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

#[tokio::test]
async fn test_get_event_by_tx_hash() {
    let pool = setup_db();

    fill_db_with_test_data(&mut pool.get().expect("expected conn"));

    let repo = create_erc20_repo(pool.clone());
    let event = event_message();
    repo.store_event(event.clone()).await.expect("store event");

    let filter = FilterBy::TxHash(event.tx_hash);
    let page = Page {
        limit: Some(10),
        offset: Some(0),
    };
    let events = repo.get_events_by(filter, page).await.expect("get event");

    assert_eq!(events.len(), 1);
    assert!(
        events
            .iter()
            .any(|v| v.tx_hash.eq(&format!("{:?}", event.tx_hash)))
    );
}

#[tokio::test]
async fn test_get_event_by_from_address() {
    let pool = setup_db();
    let repo = create_erc20_repo(pool.clone());
    let event = event_message();
    repo.store_event(event.clone()).await.expect("store event");

    let from = format!("{:?}", event.event.from);

    let filter = FilterBy::From(from.clone());
    let page = Page {
        limit: Some(10),
        offset: Some(0),
    };
    let events = repo.get_events_by(filter, page).await.expect("get event");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].from, from);
}

#[tokio::test]
async fn test_get_event_by_to_address() {
    let pool = setup_db();
    let repo = create_erc20_repo(pool.clone());
    let event = event_message();
    repo.store_event(event.clone()).await.expect("store event");

    // Filter by to address
    let filter = FilterBy::To(format!("{:?}", event.event.to));
    let page = Page {
        limit: Some(10),
        offset: Some(0),
    };
    let events = repo.get_events_by(filter, page).await.expect("get event");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].to, format!("{:?}", event.event.to));
}

#[tokio::test]
async fn test_get_event_by_block_number() {
    let pool = setup_db();
    let repo = create_erc20_repo(pool.clone());
    let event = event_message();
    repo.store_event(event.clone()).await.expect("store event");

    // Filter by block number
    let filter = FilterBy::BlockNumber(event.block_number.as_u64());
    let page = Page {
        limit: Some(10),
        offset: Some(0),
    };
    let events = repo.get_events_by(filter, page).await.expect("get event");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].block_number, event.block_number.to_string());
}

#[tokio::test]
async fn test_get_event_with_pagination() {
    let pool = setup_db();
    let repo = create_erc20_repo(pool.clone());

    fill_db_with_test_data(&mut pool.get().expect("expected conn"));

    let filter = FilterBy::Ids(vec![3, 4, 5, 6]);
    let page = Page {
        limit: Some(2),
        offset: Some(0),
    };
    let events = repo
        .get_events_by(filter.clone(), page)
        .await
        .expect("get event");

    assert_eq!(events.len(), 2);
    assert_eq!(events[0].id, 3);
    assert_eq!(events[1].id, 4);

    let page = Page {
        limit: Some(2),
        offset: Some(2),
    };
    let events = repo.get_events_by(filter, page).await.expect("get event");
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].id, 5);
    assert_eq!(events[1].id, 6);
}
