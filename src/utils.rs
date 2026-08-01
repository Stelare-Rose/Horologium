use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use uuid::{NoContext, Timestamp, Uuid};

pub fn uuidv7_from_datetime(dt: DateTime<Utc>) -> Uuid {
    let secs = dt.timestamp() as u64;
    let nanos = dt.timestamp_subsec_nanos();
    let ts = Timestamp::from_unix(NoContext, secs, nanos);
    Uuid::new_v7(ts)
}

pub fn event_path(base: &Path, start: &DateTime<Utc>) -> PathBuf {
    base.join(start.format("%Y").to_string())
        .join(start.format("%m").to_string())
        .join(start.format("%d").to_string())
}
