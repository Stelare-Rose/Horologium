use std::{collections::HashMap, fs, path::{PathBuf}};

use anyhow::anyhow;
use chrono::NaiveDate;

use crate::{compile::utils::{FileAction, compare_fingerprints}, database::Database, types::Record, utils::{event_path, fingerprint}};

enum ResolvedAction {
    Upsert {
        record: Record,
        fingerprint: u64
    },
    Delete {
        path: PathBuf
    }
}

pub fn compile_all_records(
    base_path: &PathBuf,
    database: &Database,
) -> anyhow::Result<()> {
    // TODO: Clock Checks
    let mut all_actions: Vec<FileAction> = Vec::new();

    for year_entry in fs::read_dir(base_path)? {
        let year_path = year_entry?.path();
        if !year_path.is_dir() { continue; }

        for month_entry in fs::read_dir(&year_path)? {
            let month_path = month_entry?.path();
            if !month_path.is_dir() { continue; }

            for day_entry in fs::read_dir(&month_path)? {
                let day_path = day_entry?.path();
                if !day_path.is_dir() { continue; }

                let mut actions = compile_records(database, &day_path)?;
                all_actions.append(&mut actions);
            }
        }
    }

    process_records(all_actions, database)?;
    Ok(())
}

pub fn compile_day_records(
    base_path: &PathBuf,
    database: &Database,
    day: &NaiveDate
) -> anyhow::Result<()> {
    // TODO: Clock Checks
    let path = event_path(base_path, day);
    let actions = compile_records(database, &path)?;
    process_records(actions, database)?;
    Ok(())
}

fn compile_records(
    database: &Database,
    path: &PathBuf,
) -> anyhow::Result<Vec<FileAction>> {
    let mut fingerprints: HashMap<PathBuf, u64> = HashMap::new();
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let fp = fingerprint(&path)?;
        fingerprints.insert(path.canonicalize()?, fp);
    }
    let cache_fingerprints: HashMap<PathBuf, u64> = database.get_fingerprints(&path.canonicalize()?)?;
    let actions = compare_fingerprints(fingerprints, cache_fingerprints);
    Ok(actions)
}

fn process_records(
    actions: Vec<FileAction>,
    database: &Database
) -> anyhow::Result<()> {
    // TODO: Wrap this for in a transaction
    for item in actions {
        match item {
            FileAction::Upsert { path, fingerprint } => {
                let raw = fs::read_to_string(path)?;
                let record: Record = constellation_eridanus::parse(&raw).map_err(|s: String| anyhow!(s))?.try_into()?;
                database.upsert_record(record, fingerprint)?;
            },
            FileAction::Delete { path } => {
                database.remove_record(&path)?;
            },
        }
    };
    Ok(())
}
