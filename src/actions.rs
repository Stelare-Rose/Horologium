use std::{fs, path::PathBuf};

use chrono::{DateTime, Utc};
use constellation_eridanus::{Eri, serialize};
use sanitize_filename::sanitize;
use anyhow::anyhow;
use nanoid::nanoid;

use crate::{types::{Color, Event, EventId, Project, ProjectId, ProjectState, Tag, TagId}, utils::{event_path, uuidv7_from_datetime}};

// Convenience Implementation for svc and convenience when possible to use preconfigured client
pub struct Horologium { base_path: PathBuf, device_id: String }
impl Horologium {
    pub fn start(&self, name: String, tags: Vec<TagId>, project: Option<ProjectId>, body: Option<String>, start_time: Option<DateTime<Utc>>) -> anyhow::Result<EventId> {
        start(&self.base_path, &self.device_id, name, tags, project, body, start_time)
    }
    pub fn stop(&self, name: Option<String>, body: Option<String>, start_time: Option<DateTime<Utc>>) -> anyhow::Result<EventId> {
        stop(&self.base_path, &self.device_id, name, body, start_time)
    }
}

pub fn start(
    base_path: &PathBuf,
    device_id: &str,
    name: String,
    tags: Vec<TagId>, 
    project: Option<ProjectId>,
    body: Option<String>, 
    start_time: Option<DateTime<Utc>>
) -> anyhow::Result<EventId> {
    let s = start_time.unwrap_or_else(Utc::now);
    let id = uuidv7_from_datetime(s).to_string();

    let e = Event::Active { id: EventId(id.clone()), name: name.clone(), start: s, tags, project, body };

    let eri: Eri = e.into();
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
    

    Ok(EventId(id))
}

pub fn stop (
    base_path: &PathBuf,
    device_id: &str,
    name: Option<String>,
    body: Option<String>,
    start_time: Option<DateTime<Utc>>,
) -> anyhow::Result<EventId> { 
    let s = start_time.unwrap_or_else(Utc::now);
    let id = uuidv7_from_datetime(s).to_string();

    let e = Event::Inactive { id: EventId(id.clone()), name: name.clone(), start: s, body };

    let eri: Eri = e.into();
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

    Ok(EventId(id))
}

pub fn define_tag(
    base_path: &PathBuf,
    device_id: &str,
    name: String,
    color: Vec<Color>
) -> anyhow::Result<TagId> {
    let id = format!("tag_{}", nanoid!(11));

    let t = Tag { id: id.clone(), name: name.clone(), color };

    let eri: Eri = t.into();
    let serialized: String = serialize(&eri).map_err(|e: String| anyhow!(e))?;

    // File Write - Clock first
    let path = base_path.join("Tags");
    fs::create_dir_all(&path)?;

    let sanitized_device = sanitize(device_id);
    let clock_path = path.join(format!("{sanitized_device}-clock.txt"));
    let current: u64 = fs::read_to_string(&clock_path).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0);

    fs::write(clock_path, (current + 1).to_string())?;

    let sanitized = sanitize(&name);
    let file_path = path.join(format!("{sanitized}-{id}-{sanitized_device}.eri"));

    fs::write(file_path, serialized)?;

    Ok(TagId { id, name })
}

pub fn define_project(
    base_path: &PathBuf,
    device_id: &str,
    name: String,
    color: Vec<Color>,
    body: Option<String>
) -> anyhow::Result<ProjectId> {
    let id = format!("proj_{}", nanoid!(11));

    let p = Project { id: id.clone(), name: name.clone(), state: ProjectState::Active, color, body };

    let eri: Eri = p.into();
    let serialized: String = serialize(&eri).map_err(|e: String| anyhow!(e))?;

    // File Write - Clock first
    let path = base_path.join("Projects");
    fs::create_dir_all(&path)?;

    let sanitized_device = sanitize(device_id);
    let clock_path = path.join(format!("{sanitized_device}-clock.txt"));
    let current: u64 = fs::read_to_string(&clock_path).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0);

    fs::write(clock_path, (current + 1).to_string())?;

    let sanitized = sanitize(&name);
    let file_path = path.join(format!("{sanitized}-{id}-{sanitized_device}.eri"));

    fs::write(file_path, serialized)?;

    Ok(ProjectId { id, name })
}
