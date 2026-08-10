use std::{collections::HashMap, fs, path::{PathBuf}};

use anyhow::anyhow;
use chrono::NaiveDate;

use crate::{compile::utils::{FileActions, compare_fingerprints}, database::Database, types::Record, utils::{event_path, fingerprint}};

pub fn compile_record(
    base_path: &PathBuf,
    database: &Database,
    day: &NaiveDate,
) -> anyhow::Result<()> {
    let path = event_path(base_path, day);
    let mut fingerprints: HashMap<PathBuf, u64> = HashMap::new();
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let fp = fingerprint(&path)?;
        fingerprints.insert(path.canonicalize()?, fp);
    }
    let cache_fingerprints: HashMap<PathBuf, u64> = database.get_fingerprints(&path.canonicalize()?)?;
    let actions = compare_fingerprints(fingerprints, cache_fingerprints);
    process_records(actions, database)?;
    Ok(())
}

fn process_records(
    actions: Vec<FileActions>,
    database: &Database
) -> anyhow::Result<()> {
    for item in actions {
        match item {
            FileActions::Upsert { path, fingerprint } => {
                let raw = fs::read_to_string(path)?;
                let record: Record = constellation_eridanus::parse(&raw).map_err(|s: String| anyhow!(s))?.try_into()?;
                database.upsert_record(record, fingerprint)?;
            },
            FileActions::Delete { path } => {
                database.remove_record(&path)?;
            },
        }
    };
    Ok(())
}
