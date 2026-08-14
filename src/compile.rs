use std::{fs::{create_dir_all, read_to_string}, path::PathBuf};

use anyhow::{Context, anyhow};
use constellation_eridanus::parse;

use crate::{compile::utils::delete_path_from_any_table, database::{self, Database}, types::{Project, Record, Tag}, utils::fingerprint};

mod utils;
mod records;
mod projects;
mod tags;

pub struct Compile { pub base_path: PathBuf, pub database: Database }

impl Compile {
    pub fn new(base_path: PathBuf, database: Database) -> anyhow::Result<Self> {
        create_dir_all(&base_path)
            .context("Compilation Init | could not create all directories for base_path (is base_path a proper directory?)")?;
        let canonical_path = base_path.canonicalize()
            .context("Compilation Init | could not canonicalize path")?;
        Ok(Compile { base_path: canonical_path, database })
    }
}

fn compile_path(
    compile: &Compile,
    path: &PathBuf
) -> anyhow::Result<()> {
    if path.extension().and_then(|e| e.to_str()) != Some("eri"){
        return Err(anyhow!("Compile Path | file is not an eri file"));
    }
    if !path.is_dir() && !path.exists() {
        return delete_path_from_any_table(compile, path);
    }
    let raw = read_to_string(path)?;
    let eri = parse(&raw).map_err(|s: String| anyhow!(s))?;
    match eri.schema.as_str() {
        "horologium:record" => {
            let record: Record = eri.try_into()?;
            let fp = fingerprint(path)?;
            compile.database.upsert_record(record, path, fp)?;
        },
        "horologium:tag" => {
            let tag: Tag = eri.try_into()?;
            let fp = fingerprint(path)?;
            compile.database.upsert_tag(tag, path, fp)?;
        },
        "horologium:project" => {
            let project: Project = eri.try_into()?;
            let fp = fingerprint(path)?;
            compile.database.upsert_project(project, path, fp)?;
        },
        other => {
            let dis_path = path.display();
            return Err(anyhow!("Compile Path | eri in path {dis_path} is not a valid schema. found schema {other}"))
        }
    };
    todo!()
}

