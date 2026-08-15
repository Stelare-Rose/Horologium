use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::anyhow;

use crate::{compile::{Compile, utils::{FileAction, compare_fingerprints, count_clocks}}, database::{Database, delete_clock, remove_project, upsert_clock, upsert_project}, types::{Project}, utils::fingerprint};

impl Compile {
    pub fn compile_projects(&mut self) -> anyhow::Result<()> {
        compile_projects(&self.base_path, &mut self.database)
    }
}

fn compile_projects(
    base_path: &PathBuf,
    database: &mut Database,
) -> anyhow::Result<()> {
    let projects_root = base_path.join("Projects");
    let mut actions: Vec<FileAction> = Vec::new();
    let cache_fingerprints = database.get_project_fingerprints(&projects_root)?;

    if !projects_root.is_dir() {
        let fingerprints: HashMap<PathBuf, u64> = HashMap::new();
        actions.append(&mut compare_fingerprints(fingerprints, cache_fingerprints));
        actions.push(FileAction::ClockDelete { path: projects_root });
        process_projects(actions, database)?;
        return Ok(());
    }

    let clock = count_clocks(&projects_root)?;
    let cached_clock = database.get_record_clock(&projects_root)?;

    let mut fingerprints: HashMap<PathBuf, u64> = HashMap::new();
    if clock != cached_clock {
        for entry in fs::read_dir(&projects_root)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("eri") {
                continue;
            }
            let fp = fingerprint(&path)?;
            fingerprints.insert(path, fp);
            
        }
        actions.append(&mut compare_fingerprints(fingerprints, cache_fingerprints));
        actions.push(FileAction::ClockUpsert { path: projects_root, clock });
        process_projects(actions, database)?;
    }

    Ok(())
}

fn process_projects(
    actions: Vec<FileAction>,
    database: &mut Database
) -> anyhow::Result<()> {
    let tx = database.new_transaction()?;
    for item in actions {
        match item {
            FileAction::Upsert { path, fingerprint } => {
                let raw = fs::read_to_string(&path)?;
                let project: Project = constellation_eridanus::parse(&raw).map_err(|s: String| anyhow!(s))?.try_into()?;
                upsert_project(&tx, project, &path, fingerprint)?;
            },
            FileAction::Delete { path } => {
                remove_project(&tx, &path)?;
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

