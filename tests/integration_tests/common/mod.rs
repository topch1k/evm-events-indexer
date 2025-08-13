pub mod fill_tables;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use diesel::{RunQueryDsl, sql_query};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::fs;
use std::path::PathBuf;
const DB_FILE: &str = "test.sqlite";

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn setup_db() -> Pool<ConnectionManager<SqliteConnection>> {
    if PathBuf::from(DB_FILE).exists() {
        fs::remove_file(DB_FILE).unwrap();
    }

    let manager = ConnectionManager::<SqliteConnection>::new(DB_FILE);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let mut conn = pool.get().expect("Failed to get connection for migrations");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    pool
}

fn remove_db() {
    if PathBuf::from(DB_FILE).exists() {
        fs::remove_file(DB_FILE).unwrap();
    }
}

#[test]
fn test_db_init() {
    let pool = setup_db();

    let mut conn = pool.get().expect("expected connection");

    let res = sql_query("SELECT 1;").execute(&mut conn);

    assert!(res.is_ok());

    remove_db();
}
