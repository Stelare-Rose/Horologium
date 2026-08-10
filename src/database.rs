use std::{collections::HashMap, fs, path::{Path, PathBuf}};

use anyhow::{Context, anyhow};
use rusqlite::Connection;

use crate::{types::Record};

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
    pub fn get_fingerprints(&self, path: &PathBuf) -> anyhow::Result<HashMap<PathBuf, u64>> {
        get_fingerprints(&self.conn, path)
    }
    pub fn upsert_record(&self, record: Record, fp: u64) -> anyhow::Result<()> {
        upsert_record(&self.conn, record, fp)
    }
    pub fn remove_record(&self, path: &PathBuf) -> anyhow::Result<()> {
        remove_record(&self.conn, path)
    }
    pub fn get_record_clock(&self, path: &PathBuf) -> anyhow::Result<u64> {
        get_record_clock(&self.conn, path)
    }
    pub fn get_all_record_clocks(&self) -> anyhow::Result<HashMap<PathBuf, u64>> {
        get_all_record_clocks(&self.conn)
    }
}
fn init_schema(conn: &Connection) -> anyhow::Result<()>{
    const CREATE_RECORDS: &str = include_str!("../schema/records.sql");
    const CREATE_PROJECTS: &str = include_str!("../schema/projects.sql");
    const CREATE_TAGS: &str = include_str!("../schema/tags.sql");
    const CREATE_CLOCKS: &str = include_str!("../schema/clocks.sql");

    conn.execute(CREATE_RECORDS, [])?;
    conn.execute(CREATE_PROJECTS, [])?;
    conn.execute(CREATE_TAGS, [])?;
    conn.execute(CREATE_CLOCKS, [])?;

    Ok(())
}

fn get_fingerprints(conn: &Connection, path: &PathBuf) -> anyhow::Result<HashMap<PathBuf, u64>> {
    todo!()
}

fn upsert_record(conn: &Connection, record: Record, fp: u64) -> anyhow::Result<()> {
    todo!()
}

fn remove_record(conn: &Connection, path: &PathBuf) -> anyhow::Result<()> {
    todo!()
}

fn get_record_clock(conn: &Connection, path: &PathBuf) -> anyhow::Result<u64> {
    todo!()
}

fn get_all_record_clocks(conn: &Connection) -> anyhow::Result<HashMap<PathBuf, u64>> {
    todo!()
}
