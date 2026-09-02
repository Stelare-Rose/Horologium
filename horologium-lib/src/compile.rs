use std::{fs::{create_dir_all, read_to_string}, path::PathBuf};

use anyhow::{Context, anyhow};
use constellation_eridanus::parse;

use crate::{compile::utils::delete_path_from_any_table, database::Database, types::{Project, Record, Tag}, utils::fingerprint};

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
    pub fn compile_path(&self, path: &PathBuf) -> anyhow::Result<()>{
        compile_path(&self, path)
    }
}

// Hi, I needed to leave a note here because there's a subtle design decision that I left here.
// Compile_path both does not consult the clock nor does it update the state of the clock in the database.
// This is because intentionally, the state of the clock refers to the directory, not the file.
// Compile_path is a method that is scoped to just compile the single file. Therefore,
// This method cannot claim knowledge of the whole directory. Therefore,
// The state of the clock should not be updated to claim as such.
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
    let parent = match path.parent() {
        Some(p) => {
            p
        },
        None => {
            return Err(anyhow!("Compile Path | directory has no parent?"));
        }
    };
        
    match eri.schema.as_str() {
        "horologium:record" => {
            let record: Record = eri.try_into()?;
            let fp = fingerprint(path)?;
            compile.database.upsert_record(record, path, fp)?;
            compile.database.insert_gravestone(&parent.to_path_buf())?;
        },
        "horologium:tag" => {
            let tag: Tag = eri.try_into()?;
            let fp = fingerprint(path)?;
            compile.database.upsert_tag(tag, path, fp)?;
            compile.database.insert_gravestone(&parent.to_path_buf())?;
        },
        "horologium:project" => {
            let project: Project = eri.try_into()?;
            let fp = fingerprint(path)?;
            compile.database.upsert_project(project, path, fp)?;
            compile.database.insert_gravestone(&parent.to_path_buf())?;
        },
        other => {
            let dis_path = path.display();
            return Err(anyhow!("Compile Path | eri in path {dis_path} is not a valid schema. found schema {other}"))
        }
    };
    Ok(())
}

