use std::{collections::HashMap, path::Path};

use anyhow::Context;
use rusqlite::Connection;

pub struct Reader {
    conn: Connection
}
impl Reader {
    pub fn new(db_path: &Path) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)
            .context("could not open database. Is Horologium-svc running?")?;
        Ok(Reader { conn })
    }
    pub fn get_tag_map(&self) -> anyhow::Result<HashMap<String, (String, Option<String>)>> {
        let mut stmt = self.conn.prepare("SELECT id, name, color FROM Tags")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
            ))
        })?;
        let mut map = HashMap::new();
        for r in rows {
            let (id, name, color) = r?;
            map.insert(id, (name, color));
        }
        Ok(map)
    }
}

mod timeline;
mod tags;
mod projects;
