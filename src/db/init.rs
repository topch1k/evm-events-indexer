use crate::errors::{Errors, IndexerResult};
use diesel::{SqliteConnection, r2d2::ConnectionManager, sqlite::Sqlite};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use r2d2::Pool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn init_db(db_path: &str) -> IndexerResult<Pool<ConnectionManager<SqliteConnection>>> {
    let manager = ConnectionManager::<SqliteConnection>::new(db_path);
    let pool: Pool<ConnectionManager<SqliteConnection>> = r2d2::Pool::builder().build(manager)?;

    run_migrations(&mut pool.get()?)?;

    Ok(pool)
}

fn run_migrations(conn: &mut impl MigrationHarness<Sqlite>) -> IndexerResult<()> {
    let _ = conn.run_pending_migrations(MIGRATIONS).map_err(|e| {
        log::warn!("Running migration error : {e:?}");
        Errors::RunningMigrationErrors
    })?;
    Ok(())
}
