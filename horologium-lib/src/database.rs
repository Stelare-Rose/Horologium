use std::{fs, io::ErrorKind, path::{Path, PathBuf}};

use anyhow::{Context, anyhow};
use rusqlite::{Connection, Row, Transaction, params};

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

const DATABASE_VERSION_CODE: &str = "v1.0.0";
pub struct Database {
    conn: Connection
}
impl Database {
    pub fn new(db_path: &Path) -> anyhow::Result<Self> {
        const QUERY_VERSION: &str = include_str!("../queries/metadata/get-version.sql");
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)
                .context("Actions Init | could not create all directories for base_path (is base_path a proper directory?)")?;
        }
        let mut conn = Connection::open(db_path)
            .context("Database Init | could not open database")?;
        // Current Migration Strategy: Drop the database.
        let version = conn.query_one(QUERY_VERSION, [], |row| {
            let code: String = row.get(1)?;
            Ok(code)
        }).unwrap_or("invalid version code".to_string());
        if DATABASE_VERSION_CODE != version {
            println!("Database Init | Detected Version Code Mismatch. {version} and {DATABASE_VERSION_CODE}");
            reset_db(db_path)?;
            conn = Connection::open(db_path)
                .context("Database Init | could not open database")?;
        }
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
    const CREATE_METADATA: &str = include_str!("../schema/metadata.sql");
    const CREATE_VIEWS: &str = include_str!("../schema/views.sql");
    const INIT_PRAGMA: &str = include_str!("../schema/pragma.sql");

    const SET_VERSION: &str = include_str!("../queries/metadata/set-version.sql");

    conn.execute_batch(CREATE_RECORDS)?;
    conn.execute_batch(CREATE_PROJECTS)?;
    conn.execute_batch(CREATE_TAGS)?;
    conn.execute_batch(CREATE_CLOCKS)?;
    conn.execute_batch(CREATE_METADATA)?;
    conn.execute_batch(CREATE_VIEWS)?;
    conn.execute_batch(INIT_PRAGMA)?;

    conn.execute(SET_VERSION, params!["DATABASE_VERSION_CODE", DATABASE_VERSION_CODE])?;

    Ok(())
}

fn reset_db(db_path: &Path) -> anyhow::Result<()> {
    let db_str = db_path.to_string_lossy();
    for suffix in ["", "-wal", "-shm"] {
        let path = PathBuf::from(format!("{db_str}{suffix}"));
        match std::fs::remove_file(&path) {
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => return Err(anyhow!(e)),
        }
    }
    Ok(())
}
