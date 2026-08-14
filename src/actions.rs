use std::path::PathBuf;

use anyhow::Context;

use std::fs::create_dir_all;

pub mod record;
pub mod project;
pub mod tags;

// Convenience Implementation for svc and convenience when possible to use preconfigured client
pub struct Actions { base_path: PathBuf, device_id: String }

impl Actions {
    pub fn new (base_path: PathBuf, device_id: String) -> anyhow::Result<Self> {
        create_dir_all(&base_path)?;
        let canonical_path = base_path.canonicalize()
            .context("Actions Init | could not canonicalize path")?;
        Ok(Actions { base_path: canonical_path, device_id })
    }
}
