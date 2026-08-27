use std::{collections::HashMap, path::PathBuf};

use rusqlite::{Connection, params};

use super::Database;

impl Database {
    pub fn get_record_fingerprints(
        &self,
        path: &PathBuf,
    ) -> anyhow::Result<HashMap<PathBuf, u64>> {
        get_record_fingerprints(&self.conn, path)
    }
    pub fn get_project_fingerprints(&self, path: &PathBuf) -> anyhow::Result<HashMap<PathBuf, u64>> {
        get_project_fingerprints(&self.conn, path)
    }
    pub fn get_tag_fingerprints(&self, path: &PathBuf) -> anyhow::Result<HashMap<PathBuf, u64>> {
        get_tag_fingerprints(&self.conn, path)
    }
}

fn get_record_fingerprints(
    conn: &Connection,
    path: &PathBuf,
) -> anyhow::Result<HashMap<PathBuf, u64>> {
    const GET: &str = include_str!("../../queries/fingerprints/get-records.sql");

    let mut stmt = conn.prepare(GET)?;
    let rows = stmt.query_map(params![path.to_str()], |row| {
        let path: String = row.get(0)?;
        let fp: i64 = row.get(1)?;
        Ok((PathBuf::from(path), fp))
    })?;

    let mut fingerprints = HashMap::new();
    for row in rows {
        let (path, fp) = row?;
        fingerprints.insert(path, fp as u64);
    }
    Ok(fingerprints)
}

fn get_project_fingerprints(conn: &Connection, path: &PathBuf) -> anyhow::Result<HashMap<PathBuf, u64>> {
    const GET: &str = include_str!("../../queries/fingerprints/get-projects.sql");

    let mut stmt = conn.prepare(GET)?;
    let rows = stmt.query_map(params![path.to_str()], |row| {
        let path: String = row.get(0)?;
        let fp: i64 = row.get(1)?;
        Ok((PathBuf::from(path), fp))
    })?;

    let mut fingerprints = HashMap::new();
    for row in rows {
        let (path, fp) = row?;
        fingerprints.insert(path, fp as u64);
    }
    Ok(fingerprints)
}

fn get_tag_fingerprints(conn: &Connection, path: &PathBuf) -> anyhow::Result<HashMap<PathBuf, u64>> {
     const GET: &str = include_str!("../../queries/fingerprints/get-tags.sql");

    let mut stmt = conn.prepare(GET)?;
    let rows = stmt.query_map(params![path.to_str()], |row| {
        let path: String = row.get(0)?;
        let fp: i64 = row.get(1)?;
        Ok((PathBuf::from(path), fp))
    })?;

    let mut fingerprints = HashMap::new();
    for row in rows {
        let (path, fp) = row?;
        fingerprints.insert(path, fp as u64);
    }
    Ok(fingerprints)
}
