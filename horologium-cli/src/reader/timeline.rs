use rusqlite::{Connection, params};

use crate::reader::Reader;

pub struct RecordWithTagsAndProject {
    pub id: String,
    pub name: Option<String>,
    pub status: String,
    pub start: i64,
    pub end: Option<i64>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub project_color: Option<String>,
    pub tag_ids: Option<String>,
}

impl Reader {
    pub fn get_records(
        &self,
        start: Option<i64>,
        stop: Option<i64>,
    ) -> anyhow::Result<Vec<RecordWithTagsAndProject>> {
        get_records(&self.conn, start, stop)
    }
}

pub fn get_records(
    conn: &Connection,
    start: Option<i64>,
    stop: Option<i64>,
) -> anyhow::Result<Vec<RecordWithTagsAndProject>> {
    const QUERY: &str = include_str!("../../queries/timeline/get-records.sql");
    let mut stmt = conn.prepare(QUERY)?;
    let rows = stmt.query_map(params![start, stop], |row| {
        Ok(RecordWithTagsAndProject {
            id: row.get(0)?,
            name: row.get(1)?,
            status: row.get(2)?,
            start: row.get(3)?,
            end: row.get(4)?,
            project_id: row.get(5)?,
            project_name: row.get(6)?,
            project_color: row.get(7)?,
            tag_ids: row.get(8)?,
        })
    })?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
