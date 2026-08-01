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
    let path = event_path(base_path, &s);
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
    let path = event_path(base_path, &s);
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

pub fn modify_record (
    base_path: &PathBuf,
    device_id: &str,
    from: &PathBuf,
    record: Record
) -> anyhow::Result<RecordId> {
    let path = event_path(base_path, &record.start);
    let sanitized = match record.event {
        Event::Active { ref name, .. } => {
            sanitize(&name)
        },
        Event::Inactive { ref name } => {
            sanitize(name.as_deref().unwrap_or("untracked"))
        }
    };

    let sanitized_device = sanitize(device_id);
    let id = record.id.0.clone();

    // Check if we need to rewrite a file
    let file_path = path.join(format!("{sanitized}-{id}-{sanitized_device}.eri"));

    let clock_path = path.join(format!("{sanitized_device}-clock.txt"));
    let current: u64 = fs::read_to_string(&clock_path).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0);
    fs::write(clock_path, (current + 1).to_string())?;
    
    let e: Eri = record.into();
    let serialized: String = serialize(&e).map_err(|e: String| anyhow!(e))?;

    if file_path == *from {
        // we don't
        fs::write(file_path, serialized)?;
    } else {
        // we do
        
        // update the old clock first (in case parents aren't the same)
        if file_path.parent() != from.parent() {
            let old_path = from.parent().ok_or_else(|| anyhow!("Path has no parent?"))?.to_path_buf();
            let clock_path = old_path.join(format!("{sanitized_device}-clock.txt"));
            let current: u64 = fs::read_to_string(&clock_path).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0);
            fs::write(clock_path, (current + 1).to_string())?;
        }

        // write first, removing the risk of data loss
        fs::write(file_path, serialized)?;

        fs::remove_file(from)?;
    }

    Ok(RecordId(id))
}
