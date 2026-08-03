use std::{fs, path::PathBuf};

use constellation_eridanus::{Eri, serialize};
use sanitize_filename::sanitize;
use anyhow::anyhow;
use nanoid::nanoid;

use crate::{types::{Color, Project, ProjectId, ProjectState}, utils::event_path};

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

pub fn modify_project(
    base_path: &PathBuf,
    device_id: &str,
    from: &PathBuf,
    project: Project
) -> anyhow::Result<ProjectId> {
    let path = base_path.join("Projects");
    fs::create_dir_all(&path)?;

    let sanitized = sanitize(&project.name);

    let sanitized_device = sanitize(device_id);
    let id = project.id.clone();
    let name = project.name.clone();

    let e: Eri = project.into();
    let serialized = serialize(&e).map_err(|e: String| anyhow!(e))?;

    let file_path = path.join(format!("{sanitized}-{id}-{sanitized_device}.eri"));
    let clock_path = path.join(format!("{sanitized_device}-clock.txt"));
    let current: u64 = fs::read_to_string(&clock_path).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0);

    fs::write(clock_path, (current + 1).to_string())?;

    fs::write(&file_path, serialized)?;

    if file_path != *from {
        fs::remove_file(from)?;
    };

    Ok(ProjectId { id, name })
}

pub fn complete_project(
    base_path: &PathBuf,
    device_id: &str,
    target: &PathBuf
) -> anyhow::Result<ProjectId> {

    // Probably we should store the path to the file in the sqlite db and have a consumer query it.
    // This part should just read the file, then rewrite it with the new projectstate
    todo!()
}
