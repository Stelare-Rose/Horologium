use anyhow::{Context, anyhow};
use horologium_lib::{database::Database, types::{Color, Colorscheme, ProjectId, TagId}};
use strum::IntoEnumIterator;
use std::{env, fs, process::Command};
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
