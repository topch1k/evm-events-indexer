use chrono::DateTime;
use diesel::dsl::insert_into;
use diesel::r2d2::ConnectionManager;
use diesel::{RunQueryDsl, SqliteConnection};
use logger_playground::db::models::Erc20TranferEvent;
use r2d2::PooledConnection;

pub const FROM: &str = "0xdac17f958d2ee523a2206206994597c13d831ec7";
pub const TO: &str = "110ba8f305a5ae6815b4fa123a61d94d26b4e1f7";
pub const VALUE: u64 = 1000;
pub const BLOCK_NUMBER: u64 = 1234;
pub const TX_HASH: &str = "0x378108375f820b65276b6db3a180faa0b71a28b8aa24234dd7c143db149ea3b9";
pub const LOG_INDEX: u64 = 7;

pub fn erc20_transfer_events() -> Vec<Erc20TranferEvent> {
    vec![
        Erc20TranferEvent {
            id: 3,
            from: "0x18e296053cbdf986196903e889b7dca7a73882f6".to_string(),
            to: "0xffa786da25874584377193c9302f479db0759ba5".to_string(),
            value: 500_000_000.to_string(),
            block_number: 23_095_984.to_string(),
            tx_hash: "0x954882db3c2d22a14186f5aae7164fcf1b59ebe65c5c7581135d823a06caaa79"
                .to_string(),
            log_index: 33.to_string(),
            created: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
            updated: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
        },
        Erc20TranferEvent {
            id: 4,
            from: "0x46340b20830761efd32832a74d7169b29feb9758".to_string(),
            to: "0x3a9ccb3e5bdf8e807ba19a46b4037d61d7eba65d".to_string(),
            value: 600_446_710.to_string(),
            block_number: 23_095_984.to_string(),
            tx_hash: "0x2a26b0209dd1ca116880ad38002f8dfe72fdf4d1e47b23c8078706c87df55daa"
                .to_string(),
            log_index: 50.to_string(),
            created: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
            updated: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
        },
        Erc20TranferEvent {
            id: 5,
            from: "0x185a1ff695d30a22c19f44c6b41e2d6d1c8c1f11".to_string(),
            to: "0x6e4141d33021b52c91c28608403db4a0ffb50ec6".to_string(),
            value: 1_806_421_749.to_string(),
            block_number: 23_095_984.to_string(),
            tx_hash: "0xba667170767d30da233b82fbf73ad8f75ca76c52f8d9928e113aaa88deb758fc"
                .to_string(),
            log_index: 78.to_string(),
            created: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
            updated: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
        },
        Erc20TranferEvent {
            id: 6,
            from: "0x6e4141d33021b52c91c28608403db4a0ffb50ec6".to_string(),
            to: "0xbbcb91440523216e2b87052a99f69c604a7b6e00".to_string(),
            value: 1_806_421_751.to_string(),
            block_number: 23_095_984.to_string(),
            tx_hash: "0xba667170767d30da233b82fbf73ad8f75ca76c52f8d9928e113aaa88deb758fc"
                .to_string(),
            log_index: 86.to_string(),
            created: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
            updated: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
        },
        Erc20TranferEvent {
            id: 7,
            from: "0x34524d3eeb4e422010d1915bbb27f4611010d0ba".to_string(),
            to: "0x74de5d4fcbf63e00296fd95d33236b9794016631".to_string(),
            value: 14_947_172.to_string(),
            block_number: 23_095_984.to_string(),
            tx_hash: "0xbad7f8cb0f70cf5a16bc18173c8cc48c1c535ad53ceaf1fcc73ffd971da0aa80"
                .to_string(),
            log_index: 97.to_string(),
            created: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
            updated: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
        },
        Erc20TranferEvent {
            id: 8,
            from: "0x74de5d4fcbf63e00296fd95d33236b9794016631".to_string(),
            to: "0x51c72848c68a965f66fa7a88855f9f7784502a7f".to_string(),
            value: 14_947_172.to_string(),
            block_number: 23_095_984.to_string(),
            tx_hash: "0xbad7f8cb0f70cf5a16bc18173c8cc48c1c535ad53ceaf1fcc73ffd971da0aa80"
                .to_string(),
            log_index: 98.to_string(),
            created: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
            updated: DateTime::parse_from_rfc3339("2025-08-12T21:08:11Z")
                .unwrap()
                .into(),
        },
    ]
}

pub fn fill_db_with_test_data(conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>) {
    use logger_playground::db::schema::erc20_transfer_events;
    let values = erc20_transfer_events();

    let res = insert_into(erc20_transfer_events::table)
        .values(&values)
        .execute(conn);

    assert!(res.is_ok());
    assert!(res.expect("expected inserted").eq(&values.len()));
}
