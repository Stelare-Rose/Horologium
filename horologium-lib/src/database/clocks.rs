use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use rusqlite::{Connection, params, OptionalExtension};

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
    pub fn insert_gravestone(&self, path: &PathBuf) -> anyhow::Result<()>{
        insert_gravestone(&self.conn, path)
    }
    pub fn delete_clock(&self, path: &PathBuf) -> anyhow::Result<()> {
        delete_clock(&self.conn, path)
    }
}

const GET_CLOCK: &str = include_str!("../../queries/clocks/get.sql");

fn get_record_clock(conn: &Connection, path: &PathBuf) -> anyhow::Result<u64> {
    let row: Option<i64> = conn
        .query_row(GET_CLOCK, params![path.to_str()], |row| {
            row.get::<_, i64>(0)
        })
        .optional()
        .context("Database Clock Get | failed to fetch clock")?;

    match row {
        None => Ok(0),
        Some(sum) => Ok(sum as u64),
    }
}

fn get_all_record_clocks(conn: &Connection) -> anyhow::Result<HashMap<PathBuf, u64>> {
    const GET_ALL: &str = include_str!("../../queries/clocks/get-all.sql");

    let mut stmt = conn.prepare(GET_ALL)?;
    let rows = stmt.query_map([], |row| {
        let path: String = row.get(0)?;
        let sum: i64 = row.get(1)?;
        Ok((PathBuf::from(path), sum))
    })?;

    let mut clocks = HashMap::new();
    for row in rows {
        let (path, sum) = row?;
        if path.components().any(|p| {
            let s = p.as_os_str();
            s == "Projects" || s == "Tags"
        }) {
            continue;
        }
        clocks.insert(path, sum as u64);
    }
    Ok(clocks)
}

pub fn upsert_clock(conn: &Connection, path: &PathBuf, sum: u64) -> anyhow::Result<()> {
    const UPSERT: &str = include_str!("../../queries/clocks/upsert.sql");

    conn.execute(UPSERT, params![path.to_str(), sum as i64])
        .context("Database Clock Upsert | failed to upsert clock")?;

    Ok(())
}

fn insert_gravestone(conn: &Connection, path: &PathBuf) -> anyhow::Result<()>{
    const GRAVESTONE: &str = include_str!("../../queries/clocks/gravestone.sql");

    conn.execute(GRAVESTONE, params![path.to_str()])
        .context("Database Clock Gravestone | failed to insert gravestone")?;

    Ok(())
}

pub fn delete_clock(conn: &Connection, path: &PathBuf) -> anyhow::Result<()> {
    const DELETE: &str = include_str!("../../queries/clocks/delete.sql");

    conn.execute(DELETE, params![path.to_str()])
        .context("Database Clock Delete | failed to delete clock")?;

    Ok(())
}
