use std::{fs, path::Path};

use anyhow::Context;
use rusqlite::{Connection, Row, Transaction};

mod records;
mod clocks;
mod fingerprints;
mod projects;
mod tags;
mod utils;

pub use records::{upsert_record, remove_record};
pub use clocks::{upsert_clock, delete_clock};
pub use projects::{upsert_project, remove_project};
pub use tags::{upsert_tag, remove_tag};

pub struct Database {
    conn: Connection
}
impl Database {
    pub fn new(db_path: &Path) -> anyhow::Result<Self> {
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)
                .context("Actions Init | could not create all directories for base_path (is base_path a proper directory?)")?;
        }
        let conn = Connection::open(db_path)
            .context("Database Init | could not open database")?;
        init_schema(&conn)?;
        Ok(Database { conn })
    }
    pub fn new_transaction(&mut self) -> anyhow::Result<Transaction<'_>> {
        Ok(self.conn.transaction()?)
    }
    pub fn query<T, F>(&self, sql: &str, params: &[&dyn rusqlite::ToSql], f: F) -> anyhow::Result<Vec<T>>
    where
        F: FnMut(&Row<'_>) -> rusqlite::Result<T>,
    {
        let mut stmt = self.conn
            .prepare(sql)
            .with_context(|| format!("Database Query | failed to prepare statement: {sql}"))?;
        let rows = stmt
            .query_map(params, f)
            .with_context(|| format!("Database Query | failed to execute query: {sql}"))?;
        rows.collect::<Result<Vec<T>, _>>()
            .with_context(|| format!("Database Query | failed to collect rows: {sql}"))
    }
    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> anyhow::Result<usize> {
        self.conn
            .execute(sql, params)
            .with_context(|| format!("Database Execute | failed to execute statement: {sql}"))
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
