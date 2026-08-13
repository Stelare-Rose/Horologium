use std::{collections::HashMap, path::PathBuf};

use rusqlite::Connection;

use super::Database;

impl Database {
    pub fn get_record_fingerprints(&self, day_path: &PathBuf) -> anyhow::Result<HashMap<PathBuf, u64>> {
        get_record_fingerprints(&self.conn, day_path)
    }
    pub fn get_project_fingerprints(&self) -> anyhow::Result<HashMap<PathBuf, u64>> {
        get_project_fingerprints(&self.conn)
    }
    pub fn get_tag_fingerprints(&self) -> anyhow::Result<HashMap<PathBuf, u64>> {
        get_tag_fingerprints(&self.conn)
    }
}

fn get_record_fingerprints(conn: &Connection, _day_path: &PathBuf) -> anyhow::Result<HashMap<PathBuf, u64>> {
    todo!()
}

fn get_project_fingerprints(conn: &Connection) -> anyhow::Result<HashMap<PathBuf, u64>> {
    todo!()
}

fn get_tag_fingerprints(conn: &Connection) -> anyhow::Result<HashMap<PathBuf, u64>> {
    todo!()
}