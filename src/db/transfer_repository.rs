use crate::{
    db::{
        models::{Erc20TranferEvent, NewErc20TransferEvent},
        query_filters::{FilterBy, Page},
        repository::EventRepository,
        schema::erc20_transfer_events,
    },
    errors::{Errors, IndexerResult},
    event::EventMessage,
    transfer_event::TransferEvent,
};
use diesel::{
    ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection, dsl::insert_into,
    r2d2::ConnectionManager,
};
use r2d2::Pool;

pub struct ERC20TransferRepo {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl ERC20TransferRepo {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl EventRepository for ERC20TransferRepo {
    type Err = Errors;
    type EventType = TransferEvent;
    type Filter = FilterBy;
    type EventEntity = Erc20TranferEvent;
    type Page = Page;

    async fn store_event(&self, event: EventMessage<TransferEvent>) -> IndexerResult<()> {
        log::trace!("Storing event : {event:?}");

        let new_event: NewErc20TransferEvent = event.into();
        let res = insert_into(erc20_transfer_events::table)
            .values(&new_event)
            .on_conflict_do_nothing()
            .execute(&mut self.pool.get()?)?;

        log::trace!("Inserting res : {res}",);

        Ok(())
    }

    async fn get_events_by(
        &self,
        query_filter: FilterBy,
        page: Page,
    ) -> IndexerResult<Vec<Erc20TranferEvent>> {
        log::debug!("Getting event by : {query_filter:?}");

        let query = erc20_transfer_events::table
            .select(Erc20TranferEvent::as_select())
            .into_boxed();

        let mut query = match query_filter {
            FilterBy::Ids(ids) => query.filter(erc20_transfer_events::id.eq_any(ids)),
            FilterBy::From(from) => query.filter(erc20_transfer_events::from.eq(from)),
            FilterBy::To(to) => query.filter(erc20_transfer_events::to.eq(to)),
            FilterBy::TxHash(hash) => {
                query.filter(erc20_transfer_events::tx_hash.eq(format!("{hash:?}")))
            }
            FilterBy::BlockNumber(block_number) => {
                query.filter(erc20_transfer_events::block_number.eq(block_number.to_string()))
            }
        };
        let Page { offset, limit } = page;
        if let Some(offset) = offset {
            query = query.offset(offset);
        };

        if let Some(limit) = limit {
            query = query.limit(limit)
        }

        let events = query.load::<Erc20TranferEvent>(&mut self.pool.get()?)?;

        Ok(events)
    }
}
