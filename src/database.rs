use std::{fs, path::Path};

use anyhow::Context;
use rusqlite::{Connection, Transaction};

mod records;
mod clocks;
mod fingerprints;

pub use records::{upsert_record, remove_record};
pub use clocks::{upsert_clock, delete_clock};

pub struct Database {
    conn: Connection
}
impl Database {
    pub fn new(db_path: &Path) -> anyhow::Result<Self> {
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(db_path)
            .context("Database Init | Could not open database")?;
        init_schema(&conn)?;
        Ok(Database { conn })
    }
    pub fn new_transaction(&mut self) -> anyhow::Result<Transaction<'_>> {
        Ok(self.conn.transaction()?)
    }
}
fn init_schema(conn: &Connection) -> anyhow::Result<()>{
    const CREATE_RECORDS: &str = include_str!("../schema/records.sql");
    const CREATE_PROJECTS: &str = include_str!("../schema/projects.sql");
    const CREATE_TAGS: &str = include_str!("../schema/tags.sql");
    const CREATE_CLOCKS: &str = include_str!("../schema/clocks.sql");

    conn.execute_batch(CREATE_RECORDS)?;
    conn.execute_batch(CREATE_PROJECTS)?;
    conn.execute_batch(CREATE_TAGS)?;
    conn.execute_batch(CREATE_CLOCKS)?;

    Ok(())
}