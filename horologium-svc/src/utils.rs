use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

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
pub fn default_base_path() -> PathBuf {
    dirs::data_dir()
        .expect("Could not resolve data directory")
        .join("Horologium")
}

pub fn default_db_path() -> PathBuf {
    dirs::cache_dir()
        .expect("Could not resolve cache directory")
        .join("Horologium")
        .join("main.db")
}

pub fn default_device_id() -> String {
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".to_string());
    format!("{}-{}", hostname, uuid::Uuid::new_v4())
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
            let content =
                toml::to_string_pretty(&raw).context("failed to serialize default config")?;
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
