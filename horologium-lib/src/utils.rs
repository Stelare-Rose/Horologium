use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::hash::{Hash, Hasher};

use chrono::{DateTime, NaiveDate, Utc};
use rustc_hash::FxHasher;
use uuid::{NoContext, Timestamp, Uuid};

use crate::types::Fingerprint;

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

pub fn get_stat(path: &PathBuf) -> anyhow::Result<Fingerprint> {
    let meta = fs::metadata(path)?;
    Ok(Fingerprint { 
        size: meta.size(), 
        mtime: meta.mtime(),
        mtime_nsec: meta.mtime_nsec(),
        ctime: meta.ctime(), 
        ctime_nsec: meta.ctime_nsec()
    })
}
