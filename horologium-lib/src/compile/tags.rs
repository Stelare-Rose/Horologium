use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::anyhow;

use crate::{compile::{Compile, utils::{FileAction, compare_fingerprints, count_clocks}}, database::{Database, delete_clock, remove_tag, upsert_clock, upsert_tag}, types::Tag, utils::fingerprint};

impl Compile {
    pub fn compile_tags(&mut self) -> anyhow::Result<()>{
        compile_tags(&self.base_path, &mut self.database)
    }
}

fn compile_tags(
    base_path: &PathBuf,
    database: &mut Database
) -> anyhow::Result<()> {
    let tags_root = base_path.join("Tags");
    let mut actions: Vec<FileAction> = Vec::new();
    let cache_fingerprints = database.get_tag_fingerprints(&tags_root)?;

    if !tags_root.is_dir() {
        let fingerprints: HashMap<PathBuf, u64> = HashMap::new();
        actions.append(&mut compare_fingerprints(fingerprints, cache_fingerprints));
        actions.push(FileAction::ClockDelete { path: tags_root });
        process_tags(actions, database)?;
        return Ok(());
    }

    let clock = count_clocks(&tags_root)?;
    let cached_clock = database.get_record_clock(&tags_root)?;

    let mut fingerprints: HashMap<PathBuf, u64> = HashMap::new();
    if clock != cached_clock {
        for entry in fs::read_dir(&tags_root)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("eri") {
                continue;
            }
            let fp = fingerprint(&path)?;
            fingerprints.insert(path, fp);
        }
        actions.append(&mut compare_fingerprints(fingerprints, cache_fingerprints));
        actions.push(FileAction::ClockUpsert { path: tags_root, clock });
        process_tags(actions, database)?;
    }
    Ok(())
}

fn process_tags(
    actions: Vec<FileAction>,
    database: &mut Database
) -> anyhow::Result<()> {
    let tx = database.new_transaction()?;
    for item in actions {
        match item {
            FileAction::Upsert { path, fingerprint } => {
                let raw = fs::read_to_string(&path)?;
                let tag: Tag = constellation_eridanus::parse(&raw).map_err(|s: String| anyhow!(s))?.try_into()?;
                upsert_tag(&tx, tag, &path, fingerprint)?;
            },
            FileAction::Delete { path } => {
                remove_tag(&tx, &path)?;
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
