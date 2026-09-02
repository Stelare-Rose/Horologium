use rusqlite::Connection;

use crate::reader::Reader;

pub struct TagWithColor {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
}

impl Reader {
    pub fn get_tags(&self) -> anyhow::Result<Vec<TagWithColor>> {
        get_tags(&self.conn)
    }
}

pub fn get_tags(conn: &Connection) -> anyhow::Result<Vec<TagWithColor>> {
    const QUERY: &str = include_str!("../../queries/tags/get-all.sql");
    let mut stmt = conn.prepare(QUERY)?;
    let rows = stmt.query_map([], |row| {
        Ok(TagWithColor {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
        })
    })?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
