use std::{collections::HashMap, path::{Path, PathBuf}};

use rusqlite::Connection;

pub struct Database {
    base_path: PathBuf,
    conn: Connection
}
impl Database {
    pub fn new(base_path: PathBuf, db_path: &Path) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)?;
        init_schema(&conn)?;
        Ok(Database { base_path, conn })
    }
    pub fn get_fingerprints(&self, path: &PathBuf) -> anyhow::Result<HashMap<PathBuf, u64>> {
        get_fingerprints(&self.conn, path)
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
