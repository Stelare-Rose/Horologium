use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Context;
use chrono::NaiveDate;

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
    pub fn compile_all_records(&mut self) -> anyhow::Result<()> {
        records::compile_all_records(&self.base_path, &mut self.database)
    }
    pub fn compile_day_records(&mut self, day: &NaiveDate) -> anyhow::Result<()> {
        records::compile_day_records(&self.base_path, &mut self.database, day)
    }
}

