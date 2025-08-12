diesel::table! {
    erc20_transfer_events(id){
        id -> Integer,
        from -> Text,
        to -> Text,
        value -> Text,
        block_number -> Text, //Sqlite supports only up to u64 therefore we need to use text for storing u256. Blob can be as alternative
        tx_hash -> Text,
        log_index -> Text, //Sqlite supports only up to u64 therefore we need to use text for storing u256. Blob can be as alternative
        created -> TimestamptzSqlite,
        updated -> TimestamptzSqlite,
    }
}
