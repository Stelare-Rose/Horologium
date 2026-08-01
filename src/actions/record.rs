use std::{fs, path::PathBuf};

use chrono::{DateTime, Utc};
use constellation_eridanus::{Eri, serialize};
use sanitize_filename::sanitize;
use anyhow::anyhow;

use crate::{types::{Event, ProjectId, Record, RecordId, TagId}, utils::{event_path, uuidv7_from_datetime}};

pub fn start(
    base_path: &PathBuf,
    device_id: &str,
    name: String,
    tags: Vec<TagId>, 
    project: Option<ProjectId>,
    body: Option<String>, 
    start_time: Option<DateTime<Utc>>
) -> anyhow::Result<RecordId> {
    let s = start_time.unwrap_or_else(Utc::now);
    let id = uuidv7_from_datetime(s).to_string();

    let e = Event::Active { name: name.clone(),tags, project };
    let r = Record { id: RecordId(id.clone()), start: s, event: e, body };

    let eri: Eri = r.into();
    // TODO: Change Eri to use actual error types
    let serialized: String = serialize(&eri).map_err(|e: String| anyhow!(e))?;

    // File Write
    let path = event_path(base_path, s);
    fs::create_dir_all(&path)?;

    let sanitized_device = sanitize(device_id);
    let clock_path = path.join(format!("{sanitized_device}-clock.txt"));
    let current: u64 = fs::read_to_string(&clock_path).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0);

    fs::write(clock_path, (current + 1).to_string())?;

    let sanitized = sanitize(name);
    let file_path = path.join(format!("{sanitized}-{id}-{sanitized_device}.eri"));
    fs::write(file_path, serialized)?;
    

    Ok(RecordId(id))
}

pub fn stop (
    base_path: &PathBuf,
    device_id: &str,
    name: Option<String>,
    body: Option<String>,
    start_time: Option<DateTime<Utc>>,
) -> anyhow::Result<RecordId> { 
    let s = start_time.unwrap_or_else(Utc::now);
    let id = uuidv7_from_datetime(s).to_string();

    let e = Event::Inactive { name: name.clone() };
    let r = Record { id: RecordId(id.clone()), start: s, event: e, body };

    let eri: Eri = r.into();
    let serialized: String = serialize(&eri).map_err(|e: String| anyhow!(e))?;

    // File Write
    let path = event_path(base_path, s);
    fs::create_dir_all(&path)?;

    let sanitized_device = sanitize(device_id);
    let clock_path = path.join(format!("{sanitized_device}-clock.txt"));
    let current: u64 = fs::read_to_string(&clock_path).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0);

    fs::write(clock_path, (current + 1).to_string())?;

    let sanitized = sanitize(name.unwrap_or("untracked".to_string()));
    let file_path = path.join(format!("{sanitized}-{id}-{sanitized_device}.eri"));

    fs::write(file_path, serialized)?;

    Ok(RecordId(id))
}


