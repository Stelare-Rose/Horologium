use std::{path::{PathBuf}};

use anyhow::Context;
use chrono::NaiveDate;

use crate::database::{Database};

mod utils;
mod records;

pub struct Compile { pub base_path: PathBuf, pub database: Database }

impl Compile {
    pub fn new(base_path: PathBuf, database: Database) -> anyhow::Result<Self> {
        let canonical_path = base_path.canonicalize()
            .context("Compilation Init | Could not canonicalize path")?;
        Ok(Compile { base_path: canonical_path, database })
    }
    pub fn compile_all_records(&self) -> anyhow::Result<()> {
        records::compile_all_records(&self.base_path, &self.database)
    }
    pub fn compile_day_records(&self, day: &NaiveDate) -> anyhow::Result<()> {
        records::compile_day_records(&self.base_path, &self.database, day)
    }
}

