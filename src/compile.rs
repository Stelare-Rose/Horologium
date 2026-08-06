use std::path::PathBuf;

use chrono::NaiveDate;

use crate::{database::Database, utils::event_path};

pub struct Compile { pub base_path: PathBuf, pub database: Database }

impl Compile {

}

pub fn compile_record(
    base_path: &PathBuf,
    database: &Database,
    day: &NaiveDate,
) -> anyhow::Result<()> {
    let path = event_path(base_path, day);
    todo!()
}
