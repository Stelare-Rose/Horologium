use std::{collections::HashMap, fs, path::PathBuf};

use chrono::NaiveDate;

use crate::{database::Database, utils::{event_path, fingerprint}};

pub struct Compile { pub base_path: PathBuf, pub database: Database }

impl Compile {

}

pub fn compile_record(
    base_path: &PathBuf,
    database: &Database,
    day: &NaiveDate,
) -> anyhow::Result<()> {
    let path = event_path(base_path, day);
    // Read all files in a directory
    let mut fingerprints: HashMap<PathBuf, u64> = HashMap::new();
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        // Calculate all fingerprints of said files
        let fp = fingerprint(&path)?;
        fingerprints.insert(path.canonicalize()?, fp);
    }
    // Query all fingerprints of the directory (presumably using regex on queries)
    let mut cache_fingerprints: HashMap<PathBuf, u64> = database.get_fingerprints(&path.canonicalize()?)?;
    // Compare all fingerprints from sqlite and the directory, dropping entries where fingerprints are identical

    //fuck

    // Read and reindex items that aren't identical
    // Remove all remaining entries, remembering to stitch together the enries that were in between

    // It's like removing an item from a linked list.
    
    todo!()
}
