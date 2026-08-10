use std::{path::{PathBuf}};

use chrono::NaiveDate;

use crate::{database::Database};

mod utils;
mod records;

pub struct Compile { pub base_path: PathBuf, pub database: Database }

impl Compile {
    fn compile_record(&self, day: &NaiveDate) -> anyhow::Result<()> {
        records::compile_record(&self.base_path, &self.database, day)
    }
}

