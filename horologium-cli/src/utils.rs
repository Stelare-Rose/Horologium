use anyhow::{Context, anyhow};
use horologium_lib::{database::Database, types::{Color, Colorscheme, ProjectId, TagId}};
use owo_colors::Rgb;
use serde::{Deserialize, Serialize};
use std::{env, fs, io, path::PathBuf, process::Command};
use strum::IntoEnumIterator;
use tempfile::NamedTempFile;

pub fn open_editor() -> anyhow::Result<String> {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let tmp = NamedTempFile::new()?;
    let path = tmp.path().to_path_buf();
    let status = Command::new(&editor)
        .arg(&path)
        .status()
        .with_context(|| format!("failed to launch editor: {editor}"))?;
    if !status.success() {
        return Err(anyhow!("editor exited with a non-zero status"));
    }
    let body = fs::read_to_string(&path)?;
    Ok(body.trim().to_string())
}

pub fn resolve_tags(candidates: Vec<String>, database: &Database) -> anyhow::Result<Vec<TagId>> {
    let all_tags: Vec<TagId> = database.get_all_tags()?;
    let mut results: Vec<TagId> = Vec::new();
    for c in candidates {
        if let Some(tag) = all_tags.iter().find(|tag| tag.name == c) {
            results.push(TagId { id: tag.id.clone(), name: tag.name.clone() });
        }
        let matches: Vec<_> = all_tags.iter()
            .filter(|TagId { name, .. }| strsim::jaro_winkler(name, &c) > 0.8)
            .collect();
        match matches.len() {
            0 => Err(anyhow!("unknown tag: {c}")),
            1 => {
                results.push(TagId { id: matches[0].id.clone(), name: matches[0].name.clone() });
                Ok(())
            },
            _ => Err(anyhow!("ambiguous tag {c}: matches {:?}", matches.iter().map(|TagId { name, .. }| name).collect::<Vec<_>>())),
        }?;
    };
    Ok(results)
}

pub fn resolve_project(candidate: String, database: &Database) -> anyhow::Result<ProjectId>{
    let all_projects: Vec<ProjectId> = database.get_all_active_projects()?;
    if let Some(project) = all_projects.iter().find(|project| project.name == candidate) {
        return Ok(ProjectId { id: project.id.clone(), name: project.name.clone() });
    }
    let matches: Vec<_> = all_projects.iter()
        .filter(|ProjectId { name, .. }| strsim::jaro_winkler(name, &candidate) > 0.8)
        .collect();
    match matches.len() {
        0 => Err(anyhow!("unknown project: {candidate}")),
        1 => Ok(ProjectId { id: matches[0].id.clone(), name: matches[0].name.clone() }),
        _ => Err(anyhow!("ambiguous project {candidate}: matches {:?}", matches.iter().map(|ProjectId { name, .. }| name).collect::<Vec<_>>())),
    }
}

pub fn resolve_color(candidate: String) -> anyhow::Result<Color>{
    let input = candidate.to_lowercase();
    let color = Colorscheme::iter()
        .map(|v| {
            let score = strsim::jaro_winkler(&input, &v.to_string());
            (v, score)
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .filter(|(_, score)| *score > 0.8) 
        .map(|(v, _)| v);
    match color {
        Some(c) => Ok(Color(c)),
        None => Err(anyhow!("unable to match color {candidate}"))
    }
}

pub fn resolve_body(body: Option<String>) -> anyhow::Result<Option<String>> {
    match body {
        Some(body) if body == "__OPEN_EDITOR__" => {
            Ok(Some(open_editor()?))
        }
        Some(body) => Ok(Some(body)),
        None => Ok(None)
    }
}

pub fn hex_to_rgb(hex: &str) -> Rgb {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Rgb(r, g, b)
}

pub fn default_base_path() -> PathBuf {
    dirs::data_dir().expect("Could not resolve data directory").join("Horologium")
}

pub fn default_db_path() -> PathBuf {
    dirs::cache_dir().expect("Could not resolve cache directory").join("Horologium").join("main.db")
}

pub fn default_device_id() -> String {
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".to_string());
    format!("{}-{}", hostname, uuid::Uuid::new_v4())
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct RawConfig {
    #[serde(default, rename = "device-id", alias = "device_id")]
    device_id: Option<String>,
    #[serde(default, rename = "base-path", alias = "base_path", alias = "basepath")]
    base_path: Option<PathBuf>,
    #[serde(default, rename = "db-path", alias = "db_path", alias = "dbpath")]
    db_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub device_id: String,
    pub base_path: PathBuf,
    pub db_path: PathBuf,
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .expect("Could not resolve config directory")
        .join("Horologium")
        .join("config.toml")
}

pub fn load_config() -> anyhow::Result<Config> {
    let path = config_path();
    let raw: RawConfig = match fs::read_to_string(&path) {
        Ok(content) => toml::from_str(&content).context("failed to parse config.toml")?,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let config = Config {
                device_id: default_device_id(),
                base_path: default_base_path(),
                db_path: default_db_path(),
            };
            let raw = RawConfig {
                device_id: Some(config.device_id.clone()),
                base_path: Some(config.base_path.clone()),
                db_path: Some(config.db_path.clone()),
            };
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).context("failed to create config directory")?;
            }
            let content = toml::to_string_pretty(&raw).context("failed to serialize default config")?;
            fs::write(&path, content).context("failed to write default config.toml")?;
            return Ok(config);
        }
        Err(e) => return Err(e.into()),
    };
    Ok(Config {
        device_id: raw.device_id.unwrap_or_else(default_device_id),
        base_path: raw.base_path.unwrap_or_else(default_base_path),
        db_path: raw.db_path.unwrap_or_else(default_db_path),
    })
}
