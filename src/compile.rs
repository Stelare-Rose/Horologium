use std::{path::{PathBuf}};

use chrono::NaiveDate;

use crate::{database::Database};

mod utils;
mod records;

pub struct Compile { pub base_path: PathBuf, pub database: Database }

impl Compile {
    fn compile_all_records(&self) -> anyhow::Result<()> {
        records::compile_all_records(&self.base_path, &self.database)
    }
    fn compile_day_records(&self, day: &NaiveDate) -> anyhow::Result<()> {
        records::compile_day_records(&self.base_path, &self.database, day)
    }
}

