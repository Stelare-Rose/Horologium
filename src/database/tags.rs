use std::path::PathBuf;

use anyhow::Context;
use rusqlite::{Connection, params};

use crate::types::Tag;

use super::Database;

impl Database {
    pub fn upsert_tag(&self, tag: Tag, path: &PathBuf, fp: u64) -> anyhow::Result<()> {
        upsert_tag(&self.conn, tag, path, fp)
    }
    pub fn remove_tag(&self, path: &PathBuf) -> anyhow::Result<()> {
        remove_tag(&self.conn, path)
    }
}

pub fn upsert_tag(conn: &Connection, tag: Tag, path: &PathBuf, fp: u64) -> anyhow::Result<()> {
    const UPSERT_TAG: &str = include_str!("../../queries/tags/upsert.sql");

    let color_json = serde_json::to_string(&tag.color)?;
    conn.execute(UPSERT_TAG, params![
        tag.id,
        tag.name,
        color_json,
        path.to_str(),
        fp as i64
    ]).with_context(|| format!("Database Tag Upsert | failed upsert of Tag {:?}", tag.name))?;

    Ok(())
}

pub fn remove_tag(conn: &Connection, path: &PathBuf) -> anyhow::Result<()> {
    const DELETE_TAG: &str = include_str!("../../queries/tags/delete.sql");

    conn.execute(DELETE_TAG, params![path.to_str()])
        .context("Database Tag Delete | failed to delete tag")?;

    Ok(())
}