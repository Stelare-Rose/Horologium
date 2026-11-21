use std::fs;
use std::hash::{Hash, Hasher};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow};
use chrono::{DateTime, NaiveDate, Utc};
use rustc_hash::FxHasher;
use uuid::{NoContext, Timestamp, Uuid};

use crate::types::{CanonicalHash, Fingerprint};

pub fn uuidv7_from_datetime(dt: DateTime<Utc>) -> Uuid {
    let secs = dt.timestamp() as u64;
    let nanos = dt.timestamp_subsec_nanos();
    let ts = Timestamp::from_unix(NoContext, secs, nanos);
    Uuid::new_v7(ts)
}

pub fn event_path(base: &Path, start: &NaiveDate) -> PathBuf {
    base.join("Records")
        .join(start.format("%Y").to_string())
        .join(start.format("%m").to_string())
        .join(start.format("%d").to_string())
}

pub fn fingerprint(path: &PathBuf) -> anyhow::Result<u64> {
    let mut hasher = FxHasher::default();
    let fp = get_stat(path)?;
    fp.hash(&mut hasher);
    Ok(hasher.finish())
}

#[cfg(target_os = "android")]
pub fn get_stat(path: &PathBuf) -> anyhow::Result<Fingerprint> {
    let meta = fs::metadata(path)?;
    Ok(Fingerprint {
        size: meta.size(),
        mtime: meta.mtime(),
        mtime_nsec: meta.mtime_nsec(),
        ctime: 0,
        ctime_nsec: 0,
    })
}

#[cfg(not(target_os = "android"))]
pub fn get_stat(path: &PathBuf) -> anyhow::Result<Fingerprint> {
    let meta = fs::metadata(path)?;
    Ok(Fingerprint {
        size: meta.size(),
        mtime: meta.mtime(),
        mtime_nsec: meta.mtime_nsec(),
        ctime: meta.ctime(),
        ctime_nsec: meta.ctime_nsec(),
    })
}

pub fn normalize(base_path: &Path, path: &Path) -> anyhow::Result<PathBuf> {
    let name = path
        .file_name()
        .ok_or_else(|| anyhow!("Normalize | no file name"))?;
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("Normalize | no parent"))?
        .canonicalize()
        .context("Normalize | could not canonicalize parent")?;

    let full = parent.join(name);
    if !full.starts_with(base_path) {
        return Err(anyhow!(
            "Normalize | path is outside base path? {}",
            full.display()
        ));
    }
    Ok(full)
}

pub fn hash_of<T: CanonicalHash + ?Sized>(value: &T) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    value.write_canonical(&mut hasher);
    *hasher.finalize().as_bytes()
}
