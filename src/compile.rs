use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Context;

use crate::database::{Database};

mod utils;
mod records;
mod projects;
mod tags;

pub struct Compile { pub base_path: PathBuf, pub database: Database }

impl Compile {
    pub fn new(base_path: PathBuf, database: Database) -> anyhow::Result<Self> {
        create_dir_all(&base_path)
            .context("Compilation Init | could not create all directories for base_path (is base_path a proper directory?)")?;
        let canonical_path = base_path.canonicalize()
            .context("Compilation Init | could not canonicalize path")?;
        Ok(Compile { base_path: canonical_path, database })
    }
}
