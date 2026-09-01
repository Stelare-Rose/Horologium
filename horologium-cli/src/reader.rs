use std::path::Path;

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
}

mod timeline;
