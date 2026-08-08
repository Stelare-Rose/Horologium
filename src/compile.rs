use std::{collections::HashMap, fs, path::{PathBuf}};

use chrono::NaiveDate;

use crate::{database::Database, utils::{event_path, fingerprint}};

pub struct Compile { pub base_path: PathBuf, pub database: Database }

impl Compile {

}

enum FileActions {
    Upsert {
        path: PathBuf,
        fingerprint: u64
    },
    Delete {
        path: PathBuf
    }
}

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
    let mut cache_fingerprints: HashMap<PathBuf, u64> = database.get_fingerprints(&path.canonicalize()?)?;

    // Compare all fingerprints from sqlite and the directory, dropping entries where fingerprints are identical
    let mut actions: Vec<FileActions> = Vec::new();
    for (path, fp) in fingerprints {
        match cache_fingerprints.remove(&path) {
            Some(cache_fp) if cache_fp != fp => {
                actions.push(FileActions::Upsert { path, fingerprint: fp });
            },
            Some(_) => {
                // fingerprints are the same, we skip it here
            }
            None => { 
                actions.push(FileActions::Upsert { path, fingerprint: fp });
            }
        }
    }

    // Anything remaining in cache_fingerprints are deletes.
    for path in cache_fingerprints.into_keys() {
        actions.push(FileActions::Delete { path });
    }

    // Read and reindex items that aren't identical
    // Remove all remaining entries, remembering to stitch together the enries that were in between

    // It's like removing an item from a linked list.
    
    todo!()
}
