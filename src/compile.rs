use std::{collections::HashMap, fs, path::{PathBuf}};

use chrono::NaiveDate;

use crate::{compile::utils::{FileActions, compare_fingerprints}, database::Database, utils::{event_path, fingerprint}};

pub mod utils;

pub struct Compile { pub base_path: PathBuf, pub database: Database }

impl Compile {

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
    let cache_fingerprints: HashMap<PathBuf, u64> = database.get_fingerprints(&path.canonicalize()?)?;
    let actions = compare_fingerprints(fingerprints, cache_fingerprints);
    process_records(actions)?;

    // Compare all fingerprints from sqlite and the directory, dropping entries where fingerprints are identical


    // Read and reindex items that aren't identical
    // Remove all remaining entries, remembering to stitch together the enries that were in between

    // It's like removing an item from a linked list.
    
    Ok(())
}

fn process_records(
    actions: Vec<FileActions>
) -> anyhow::Result<()> {
    todo!()
}
