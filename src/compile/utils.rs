use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::anyhow;

pub enum FileAction {
    Upsert {
        path: PathBuf,
        fingerprint: u64
    },
    Delete {
        path: PathBuf
    },
    ClockUpsert {
        path: PathBuf,
        clock: u64
    },
    ClockDelete {
        path: PathBuf,
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

pub fn count_clocks(
    path: &PathBuf
) -> anyhow::Result<u64> { 
    if !path.is_dir() {
        return Err(anyhow!("Path is not a directory"));
    }
    let clocks = find_clock_files(path)?;
    let mut sum: u64 = 0;
    for c in clocks {
        let i: u64 = fs::read_to_string(&c).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0);
        sum += i;
    }
    Ok(sum)
}

fn find_clock_files(path: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let mut clock_files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.extension().and_then(|e| e.to_str()) == Some("clock") {
            clock_files.push(file_path);
        }
    }

    Ok(clock_files)
}
