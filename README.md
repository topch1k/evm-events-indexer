# README.md

## Introduction
This project is a simple events indexer. It uses ABI definitions passed through the configuration file to listen for blockchain events and stores the received event data in a database.

## Demo Video

Watch a demo of the indexer in action:

[![Demo Video](https://img.youtube.com/vi/huiMptFIC18/0.jpg)](https://youtu.be/huiMptFIC18)

## Example Usage

### Start Listening for Events

```sh
./logger_playground --config-path=examples/example_config.yaml start
```

### Query Stored Events with Filters

#### Filter by IDs
```sh
./logger_playground --config-path=examples/example_config.yaml list-by ids 1 2 3
```

#### Filter by Sender (from) with Pagination
```sh
./logger_playground --config-path=examples/example_config.yaml list-by --offset=1 --limit=2 from 0x18e296053cbdf986196903e889b7dca7a73882f6
```

#### Filter by Receiver (to)
```sh
./logger_playground --config-path=examples/example_config.yaml list-by to 0x31a4c778418c309d155d86519fa751e1fc78202d
```

#### Filter by Transaction Hash
```sh
./logger_playground --config-path=examples/example_config.yaml list-by tx-hash 0x300197720261fc01660095e9d1c1381e9a7e838294a64f033a29934d61be2b62
```

#### Filter by Block Number
```sh
./logger_playground --config-path=examples/example_config.yaml list-by block-number 23141680
```

## Log Indexer Approach


The log indexer is implemented in `src/indexer.rs` and related modules. Key aspects of the design:

### Generic LogIndexer and Provider Flexibility
- **LogIndexer** is a generic struct that depends on a `Provider` implementing the `Middleware` trait (from the `ethers` crate). This allows you to use custom providers, making the indexer highly adaptable to different environments or requirements.
- The current implementation is founded on a subscription model, so the provider must also implement the `PubSub` trait. This enables efficient event streaming and real-time log processing.
- Using a subscription-based approach can be more efficient for log/event indexing compared to polling.

### Flexible Event Consumption
- The usage of the `ConsumeEvent` trait provides great flexibility: any consumer can be used, such as:
	- A simple consumer that prints logs to the console.
	- A consumer-adapter that transfers logs to a message broker.
	- A consumer that stores logs in a database.

The current implementation contains a consumer that stores received logs in the database. However, this can be easily changed to any other consumer by implementing the `ConsumeEvent` trait.

### Approach Steps
1. **Event Consumption**: The indexer uses the `ConsumeEvent` trait to pass received log to another party for processing.
2. **Event Parsing**: Log parsing and all related actions are located inside implementations of `ConsumeEvent` or some other service which will receive log.
3. **Database Insertion**: Parsed events are inserted into the database using the repository pattern. 
4. **Migration Support**: Database schema migrations are managed in the `migrations/` directory, allowing for schema evolution.
5. **CLI Integration**: The CLI provides commands to start listening logs or getting received logs from db.

### Design Principles
- **Modularity**: Separation of concerns between CLI, database, event parsing, and interfaces.
- **Extensibility**: New event types or sources can be added by implementing the relevant traits and extending the repository layer.

### Extensibility and Multi-Event Support
- The current implementation can be easily improved to support multiple event types. The configuration file could be extended to specify several different events to listen for.
- With such an improvement, after parsing the configuration, several indexers could be instantiated, each with its own event type and consumer implementation.
- This would allow the system to index and process various event types in parallel, each with custom logic for handling and storing events.

### Finality and Reorgs
- For listening to only finalized blocks the according block number alias `finalized` can be used. Example can be found in `examples/example_config.yaml`
- Logs that were removed because of reorgs have specific flag `removed` and just skipped from storing in db. 

### Event Structures
### Database Layer

- For database querying, Diesel ORM is used. Diesel provides strong type safety guarantees at compile-time, reducing runtime errors and improving reliability.
- One limitation of Diesel is its synchronous nature, which can block threads during database operations. This issue can be mitigated by using the async version of Diesel, which enables asynchronous communication with the database for better performance.

### Database Schema
- Database schema updates are managed using the `diesel-migrations` crate.
- The current implementation defines a single table for storing ERC20 transfer events. This table includes:
	- `from`: Sender address
	- `block_number`: Block number containing the transaction
    - `created` and `updated`: Information about time when event was created/updated
- For updating `updated` field, trigger was created. It updates field value with row updates. 
- The schema could be enhanced by using custom types, such as a custom `hash` type instead of `Text` or a custom `U256` type for values. This would provide stronger type safety guarantees and reduce the risk of data inconsistencies.

- **CLI Commands**: Defined in `src/cli/commands.rs`.
- **Config**: `src/cli/config.rs` manages configuration loading and parsing.

### CLI Interaction
- A simple CLI was created for interacting with the service, using the `clap` framework.
	- Start listening for events
- There are several filter options available for retrieving events:
	- Filter by transaction hash
	- Filter by receiver or sender
	- Filter by block number
	- Filter by event id

### Interfaces
- **ConsumeEvent**: Trait in `src/interfaces/consume_event.rs` for consuming events from external sources.
---




