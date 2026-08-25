use rusqlite::Connection;

use crate::{database::Database, types::{ProjectId, TagId}};

impl Database {
    pub fn get_all_tags(&self) -> anyhow::Result<Vec<TagId>> {
        get_all_tags(&self.conn)
    }
    pub fn get_all_active_projects(&self) -> anyhow::Result<Vec<ProjectId>> {
        get_all_active_projects(&self.conn)
    }
}

fn get_all_tags(
    conn: &Connection
) -> anyhow::Result<Vec<TagId>> {
    const GET: &str = include_str!("../../queries/tags/get-all.sql");
    
    let mut stmt = conn.prepare(GET)?;
    let rows = stmt.query_map([], |row| {
        let id: String = row.get(0)?;
        let name: String = row.get(1)?;
        Ok(TagId{id, name})
    })?;
    
    let mut result: Vec<TagId> = Vec::new();
    for row in rows {
        let tag = row?;
        result.push(tag);
    };
    Ok(result)
}

fn get_all_active_projects(
    conn: &Connection
) -> anyhow::Result<Vec<ProjectId>> {
    const GET: &str = include_str!("../../queries/projects/get-active.sql");

    let mut stmt = conn.prepare(GET)?;
    let rows = stmt.query_map([], |row| {
        let id: String = row.get(0)?;
        let name: String = row.get(1)?;
        Ok(ProjectId{id, name})
    })?;

    let mut result: Vec<ProjectId> = Vec::new();
    for row in rows {
        let project = row?;
        result.push(project);
    };
    Ok(result)
}
