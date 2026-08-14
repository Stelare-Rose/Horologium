use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Context;

use crate::database::{Database};

mod utils;
mod records;

pub struct Compile { pub base_path: PathBuf, pub database: Database }

impl Compile {
    pub fn new(base_path: PathBuf, database: Database) -> anyhow::Result<Self> {
        create_dir_all(&base_path)?;
        let canonical_path = base_path.canonicalize()
            .context("Compilation Init | Could not canonicalize path")?;
        Ok(Compile { base_path: canonical_path, database })
    }
}
