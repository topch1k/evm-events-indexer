CREATE INDEX erc20_transfer_events_from_index
ON erc20_transfer_events("from");

CREATE INDEX erc20_transfer_events_to_index
ON erc20_transfer_events("to");

CREATE INDEX erc20_transfer_events_block_number_index
ON erc20_transfer_events(block_number);

CREATE INDEX erc20_transfer_events_tx_hash_index
ON erc20_transfer_events(tx_hash);
