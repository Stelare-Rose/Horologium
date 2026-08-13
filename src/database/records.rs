use std::path::PathBuf;

use anyhow::Context;
use rusqlite::{Connection, params, types::Null};

use crate::types::{Event, Record};

use super::Database;

impl Database {
    pub fn upsert_record(&self, record: Record, path: &PathBuf, fp: u64) -> anyhow::Result<()> {
        upsert_record(&self.conn, record, path, fp)
    }
    pub fn remove_record(&self, path: &PathBuf) -> anyhow::Result<()> {
        remove_record(&self.conn, path)
    }
}

pub fn upsert_record(conn: &Connection, record: Record, path: &PathBuf, fp: u64) -> anyhow::Result<()> {
    const UPDATE_NEXT: &str = include_str!("../../queries/records/upsert-next.sql");
    const UPDATE_PREV: &str = include_str!("../../queries/records/upsert-prev.sql");
    const UPSERT_RECORD: &str = include_str!("../../queries/records/upsert.sql");
    const INSERT_RECORD_TAG: &str = include_str!("../../queries/records/insert-tags.sql");

    let record_id = record.id.0;
    let start_epoch = record.start.timestamp();

    match record.event {
        Event::Active { name, tags, project } => {
            let project_id = project.map(|p| p.id);
            conn.execute(UPSERT_RECORD, params![
                record_id, 
                name, 
                "active", 
                start_epoch, 
                project_id, 
                path.to_str(), 
                fp as i64
            ]).with_context(|| format!("Database Record Upsert | Failed Upsert of Record {:?}", name))?;
            let mut stmt = conn.prepare(INSERT_RECORD_TAG)?;
            for (i, tag) in tags.iter().enumerate() {
                stmt.execute(params![
                    tag.id,
                    record_id,
                    i as i64
                ]).with_context(|| format!("Database Record Upsert | Failed Tag Upsert of Record {:?}, with tag {:?}", name, tag.name))?;
            }
        }
        Event::Inactive { name } => {
            conn.execute(UPSERT_RECORD, params![
                record_id,
                name,
                "inactive",
                start_epoch,
                Null,
                path.to_str(),
                fp as i64
            ]).with_context(|| format!("Database Record Upsert | Failed Upsert of Record {:?}", name))?;
        }
    };

    // Update adjacent records
    conn.execute(UPDATE_PREV, params![start_epoch]).with_context(|| format!("Database Record Upsert | Failed Previous update of Record {:?}", record_id))?;
    conn.execute(UPDATE_NEXT, params![start_epoch, record_id]).with_context(|| format!("Database Record Upsert | Failed Next update of Record {:?}", record_id))?;
    Ok(())
}

pub fn remove_record(conn: &Connection, path: &PathBuf) -> anyhow::Result<()> {
    const GET_START: &str = include_str!("../../queries/records/get-start.sql");
    const DELETE_STITCH: &str = include_str!("../../queries/records/delete-stitch.sql");
    const DELETE_RECORD: &str = include_str!("../../queries/records/delete.sql");

    let start: i64 = conn.query_row(GET_START, params![path.to_str()], |row| row.get(0))?;

    conn.execute(DELETE_STITCH, params![start])?;
    conn.execute(DELETE_RECORD, params![path.to_str()])?;
    Ok(())
}
