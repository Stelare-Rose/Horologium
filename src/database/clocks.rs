use std::{collections::HashMap, path::PathBuf};

use rusqlite::Connection;

use super::Database;

impl Database {
    pub fn get_record_clock(&self, path: &PathBuf) -> anyhow::Result<u64> {
        get_record_clock(&self.conn, path)
    }
    pub fn get_all_record_clocks(&self) -> anyhow::Result<HashMap<PathBuf, u64>> {
        get_all_record_clocks(&self.conn)
    }
    pub fn upsert_clock(&self, path: &PathBuf, sum: u64) -> anyhow::Result<()> {
        upsert_clock(&self.conn, path, sum)
    }
    pub fn delete_clock(&self, path: &PathBuf) -> anyhow::Result<()> {
        delete_clock(&self.conn, path)
    }
}

fn get_record_clock(conn: &Connection, path: &PathBuf) -> anyhow::Result<u64> {
    todo!()
}

fn get_all_record_clocks(conn: &Connection) -> anyhow::Result<HashMap<PathBuf, u64>> {
    todo!()
}

pub fn upsert_clock(conn: &Connection, path: &PathBuf, sum: u64) -> anyhow::Result<()> {
    todo!()
}

pub fn delete_clock(conn: &Connection, path: &PathBuf) -> anyhow::Result<()> {
    todo!()
}