use include_dir::{Dir, include_dir};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite_migration::Migrations;
use std::sync::LazyLock;

use crate::errors::IndexerResult;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

static MIGRATIONS: LazyLock<Migrations<'static>> =
    LazyLock::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());

pub fn init_db(db_path: &str) -> IndexerResult<Pool<SqliteConnectionManager>> {
    let pool = Pool::new(SqliteConnectionManager::file(db_path))?;
    let mut conn = pool.get()?;
    MIGRATIONS.to_latest(&mut conn)?;

    Ok(pool)
}
