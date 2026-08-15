use std::{collections::HashMap, fs, path::{PathBuf}};

use anyhow::anyhow;
use chrono::NaiveDate;

use crate::{compile::{utils::{FileAction, compare_fingerprints, count_clocks}, Compile}, database::{Database, delete_clock, remove_record, upsert_clock, upsert_record}, types::Record, utils::{event_path, fingerprint}};

impl Compile {
    pub fn compile_all_records(&mut self) -> anyhow::Result<()> {
        compile_all_records(&self.base_path, &mut self.database)
    }
    pub fn compile_day_records(&mut self, day: &NaiveDate) -> anyhow::Result<()> {
        compile_day_records(&self.base_path, &mut self.database, day)
    }
}

fn compile_all_records(
    base_path: &PathBuf,
    database: &mut Database,
) -> anyhow::Result<()> {
    let mut clocks = database.get_all_record_clocks()?;
    let mut all_actions: Vec<FileAction> = Vec::new();

    let records_root = base_path.join("Records");
    if !records_root.is_dir() {
        return Ok(());
    }

    for year_entry in fs::read_dir(&records_root)? {
        let year_path = year_entry?.path();
        if !year_path.is_dir() { continue; }

        for month_entry in fs::read_dir(&year_path)? {
            let month_path = month_entry?.path();
            if !month_path.is_dir() { continue; }

            for day_entry in fs::read_dir(&month_path)? {
                let day_path = day_entry?.path();
                if !day_path.is_dir() { continue; }
                
                let clock_sum = count_clocks(&day_path)?;
                let is_clock_same = match clocks.remove(&day_path) {
                    Some(i) => { clock_sum == i }
                    None => { false }
                };
                
                if !is_clock_same {
                    // We only add the actions if the clock is not the same
                    let mut actions = compile_records(database, &day_path)?;
                    all_actions.append(&mut actions);
                    // Update clock in database
                    all_actions.push(
                        FileAction::ClockUpsert { 
                            path: day_path, 
                            clock: clock_sum
                        }
                    );
                }
            }
        }
    }

    // Any remaining clocks are from directories that don't exist, clean up db
    for c in clocks.into_keys() {
        all_actions.push(
            FileAction::ClockDelete { 
                path: c 
            }
        );
    }

    process_records(all_actions, database)?;
    Ok(())
}

fn compile_day_records(
    base_path: &PathBuf,
    database: &mut Database,
    day: &NaiveDate
) -> anyhow::Result<()> {
    let path = event_path(base_path, day);
    let mut actions: Vec<FileAction> = Vec::new();

     if !path.is_dir() {
        // directory doesn't exist, clean up its clock and corresponding directory
        actions.append(&mut compile_records(database, &path)?);
        actions.push(FileAction::ClockDelete { path: path.clone() });
        process_records(actions, database)?;
        Ok(())
    } else {
        let cached_clock = database.get_record_clock(&path)?;
        let clock = count_clocks(&path)?;

        if clock != cached_clock {
            actions.append(&mut compile_records(database, &path)?);
            actions.push(FileAction::ClockUpsert { path: path, clock });
            process_records(actions, database)?;
        }
        Ok(())
    }
}

fn compile_records(
    database: &mut Database,
    path: &PathBuf,
) -> anyhow::Result<Vec<FileAction>> {
    let mut fingerprints: HashMap<PathBuf, u64> = HashMap::new();
    // If not directory, fall through and let all fingerprints go to delete
    if path.is_dir() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("eri") {
                continue;
            }
            let fp = fingerprint(&path)?;
            fingerprints.insert(path, fp);
        }
    }
    let cache_fingerprints: HashMap<PathBuf, u64> = database.get_record_fingerprints(&path)?;
    let actions = compare_fingerprints(fingerprints, cache_fingerprints);
    Ok(actions)
}

fn process_records(
    actions: Vec<FileAction>,
    database: &mut Database
) -> anyhow::Result<()> {
    let tx = database.new_transaction()?;
    for item in actions {
        match item {
            FileAction::Upsert { path, fingerprint } => {
                let raw = fs::read_to_string(&path)?;
                let record: Record = constellation_eridanus::parse(&raw).map_err(|s: String| anyhow!(s))?.try_into()?;
                upsert_record(&tx, record, &path, fingerprint)?;
            },
            FileAction::Delete { path } => {
                remove_record(&tx, &path)?;
            },
            FileAction::ClockUpsert { path, clock } => {
                upsert_clock(&tx, &path, clock)?;
            },
            FileAction::ClockDelete { path } => {
                delete_clock(&tx, &path)?;
            }
        }
    };
    tx.commit()?;
    Ok(())
}
