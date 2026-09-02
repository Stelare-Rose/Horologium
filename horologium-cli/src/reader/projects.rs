use rusqlite::Connection;

use crate::reader::Reader;

pub struct ProjectWithColor {
    pub id: String,
    pub name: String,
    pub state: Option<String>,
    pub color: Option<String>,
}

impl Reader {
    pub fn get_projects(&self) -> anyhow::Result<Vec<ProjectWithColor>> {
        get_projects(&self.conn)
    }
}

pub fn get_projects(conn: &Connection) -> anyhow::Result<Vec<ProjectWithColor>> {
    const QUERY: &str = include_str!("../../queries/projects/get-all.sql");
    let mut stmt = conn.prepare(QUERY)?;
    let rows = stmt.query_map([], |row| {
        Ok(ProjectWithColor {
            id: row.get(0)?,
            name: row.get(1)?,
            state: row.get(2)?,
            color: row.get(3)?,
        })
    })?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
