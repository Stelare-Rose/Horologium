use std::{fs::{self}, path::PathBuf};

use constellation_eridanus::{Eri, serialize};
use sanitize_filename::sanitize;
use anyhow::anyhow;
use nanoid::nanoid;

use crate::types::{Color, Tag, TagId};
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
    let clock_path = path.join(format!("{sanitized_device}.clock"));
    let current: u64 = fs::read_to_string(&clock_path).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0);

    fs::write(clock_path, (current + 1).to_string())?;

    let sanitized = sanitize(&name);
    let file_path = path.join(format!("{sanitized}-{id}-{sanitized_device}.eri"));

    fs::write(file_path, serialized)?;

    Ok(TagId { id, name })
}

pub fn modify_tags(
    base_path: &PathBuf,
    device_id: &str,
    from: &PathBuf,
    tags: Tag,
) -> anyhow::Result<TagId> {
    let path = base_path.join("Tags");
    fs::create_dir_all(&path)?;

    let sanitized = sanitize(&tags.name);

    let sanitized_device = sanitize(device_id);
    let id = tags.id.clone();
    let name = tags.name.clone();

    let e: Eri = tags.into();
    let serialized = serialize(&e).map_err(|e: String| anyhow!(e))?;

    let file_path = path.join(format!("{sanitized}-{id}-{sanitized_device}.eri"));
    let clock_path = path.join(format!("{sanitized_device}.clock"));
    let current: u64 = fs::read_to_string(&clock_path).ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0);

    fs::write(clock_path, (current + 1).to_string())?;

    fs::write(&file_path, serialized)?;

    if file_path != *from {
        fs::remove_file(from)?;
    };

    Ok(TagId { id, name })
}
