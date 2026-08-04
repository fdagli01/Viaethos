pub mod repo;

use rusqlite::Connection;
use std::sync::Mutex;

pub struct DbState(pub Mutex<Connection>);

const MIGRATION_0001: &str = include_str!("../../migrations/0001_init.sql");
const MIGRATION_0002: &str = include_str!("../../migrations/0002_synthetic_flag.sql");

pub fn init(app_data_dir: &std::path::Path) -> Connection {
    std::fs::create_dir_all(app_data_dir).expect("create app data dir");
    let db_path = app_data_dir.join("via-ethos.sqlite3");
    let conn = Connection::open(db_path).expect("open sqlite db");
    conn.pragma_update(None, "foreign_keys", true).ok();
    conn.execute_batch(MIGRATION_0001).expect("run migrations");
    conn.execute_batch(MIGRATION_0002).expect("run migrations");
    repo::seed_if_empty(&conn).expect("seed default pillars/actions");
    repo::seed_food_items_if_empty(&conn).expect("seed default food items");
    conn
}
