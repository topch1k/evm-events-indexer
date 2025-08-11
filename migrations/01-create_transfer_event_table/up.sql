CREATE TABLE erc20_transfer_events(
    id INTEGER PRIMARY KEY,
    "from" TEXT NOT NULL,
    "to" TEXT NOT NULL,
    "value" REAL NOT NULL,
    block_number BIGINT NOT NULL,
    tx_hash TEXT NOT NULL,
    log_index INT NOT NULL,
    created TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tx_hash, log_index)
);
-- TODO: 
-- 1. Add triggers for created, updated
-- 2. Add index according to get requests