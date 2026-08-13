use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Context;
use chrono::{DateTime, Utc};

use crate::types::{Color, Project, ProjectId, Record, RecordId, Tag, TagId};

pub mod record;
pub mod project;
pub mod tags;

// Convenience Implementation for svc and convenience when possible to use preconfigured client
pub struct Actions { base_path: PathBuf, device_id: String }

impl Actions {
    pub fn new (base_path: PathBuf, device_id: String) -> anyhow::Result<Self> {
        create_dir_all(&base_path)?;
        let canonical_path = base_path.canonicalize()
            .context("Actions Init | Could not canonicalize path")?;
        Ok(Actions { base_path: canonical_path, device_id })
    }
    pub fn start(&self, name: String, tags: Vec<TagId>, project: Option<ProjectId>, body: Option<String>, start_time: Option<DateTime<Utc>>) -> anyhow::Result<RecordId> {
        record::start(&self.base_path, &self.device_id, name, tags, project, body, start_time)
    }
    pub fn stop(&self, name: Option<String>, body: Option<String>, start_time: Option<DateTime<Utc>>) -> anyhow::Result<RecordId> {
        record::stop(&self.base_path, &self.device_id, name, body, start_time)
    }
    pub fn modify_record(&self, from: &PathBuf, record: Record) -> anyhow::Result<RecordId>{
        record::modify_record(&self.base_path, &self.device_id, from, record)
    }
    pub fn define_tag(&self, name: String, color: Vec<Color>) -> anyhow::Result<TagId> {
        tags::define_tag(&self.base_path, &self.device_id, name, color)
    }
    pub fn modify_tag(&self, from: &PathBuf, tags: Tag) -> anyhow::Result<TagId> {
        tags::modify_tags(&self.base_path, &self.device_id, from, tags)
    }
    pub fn define_project(&self, name: String, color: Vec<Color>, body: Option<String>) -> anyhow::Result<ProjectId> {
        project::define_project(&self.base_path, &self.device_id, name, color, body)
    }
    pub fn modify_project(&self, from: &PathBuf, project: Project) -> anyhow::Result<ProjectId> {
        project::modify_project(&self.base_path, &self.device_id, from, project)
    }
    pub fn complete_project(&self, target: &PathBuf) -> anyhow::Result<ProjectId> {
        project::complete_project(&self.base_path, &self.device_id, target)
    }
}


