use std::{collections::HashMap, path::PathBuf};

pub enum FileAction {
    Upsert {
        path: PathBuf,
        fingerprint: u64
    },
    Delete {
        path: PathBuf
    }
}

pub fn compare_fingerprints(
    fp: HashMap<PathBuf, u64>,
    mut c_fp: HashMap<PathBuf, u64>
) -> Vec<FileAction> {
    let mut actions: Vec<FileAction> = Vec::new();
    for (path, fp) in fp {
        match c_fp.remove(&path) {
            Some(cache_fp) if cache_fp != fp => {
                actions.push(FileAction::Upsert { path, fingerprint: fp });
            },
            Some(_) => {
                // fingerprints are the same, we skip it here
            }
            None => { 
                actions.push(FileAction::Upsert { path, fingerprint: fp });
            }
        }
    }

    // Anything remaining in cache_fingerprints are deletes.
    for path in c_fp.into_keys() {
        actions.push(FileAction::Delete { path });
    }
    actions
}

