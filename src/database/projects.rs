use std::path::PathBuf;

use anyhow::Context;
use rusqlite::{Connection, params};

use crate::types::{Project, ProjectState};

use super::Database;

impl Database {
    pub fn upsert_project(&self, project: Project, path: &PathBuf, fp: u64) -> anyhow::Result<()> {
        upsert_project(&self.conn, project, path, fp)
    }
    pub fn remove_project(&self, path: &PathBuf) -> anyhow::Result<()> {
        remove_project(&self.conn, path)
    }
}

pub fn upsert_project(conn: &Connection, project: Project, path: &PathBuf, fp: u64) -> anyhow::Result<()> {
    const UPSERT_PROJECT: &str = include_str!("../../queries/projects/upsert.sql");

    let state = match project.state {
        ProjectState::Active => "active",
        ProjectState::Done { .. } => "done",
    };
    let completed_date = match project.state {
        ProjectState::Done { completed_date } => {
            Some(completed_date.format("%Y-%m-%d").to_string())
        },
        ProjectState::Active => None,
    };
    let color_json = serde_json::to_string(&project.color)?;
    conn.execute(UPSERT_PROJECT, params![
        project.id,
        project.name,
        state,
        color_json,
        completed_date,
        path.to_str(),
        fp as i64
    ]).with_context(|| format!("Database Project Upsert | failed upsert of Project {:?}", project.name))?;

    Ok(())
}

pub fn remove_project(conn: &Connection, path: &PathBuf) -> anyhow::Result<()> {
    const DELETE_PROJECT: &str = include_str!("../../queries/projects/delete.sql");

    conn.execute(DELETE_PROJECT, params![path.to_str()])
        .context("Database Project Delete | failed to delete project")?;

    Ok(())
}
