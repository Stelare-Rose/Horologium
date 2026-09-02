use std::collections::HashMap;

use chrono::{DateTime, Utc};
use clap::{Args, Subcommand};
use horologium_lib::types::{Color, Mode};
use owo_colors::OwoColorize;

use crate::{reader::Reader, utils::hex_to_rgb};

#[derive(Args, Debug)]
pub struct DisplayArgs {
    #[command(subcommand)]
    pub command: DisplayCommands,
}

#[derive(Subcommand, Debug)]
pub enum DisplayCommands {
    Records(RecordsArgs),
    Tags,
    Projects,
}

#[derive(Args, Debug)]
pub struct RecordsArgs {
    #[arg(long)]
    pub start: Option<String>,
    #[arg(long)]
    pub end: Option<String>,
}

pub fn handle_display(args: DisplayArgs, reader: &Reader) -> anyhow::Result<()> {
    match args.command {
        DisplayCommands::Records(r) => {
            let start = r.start.as_deref().map(parse_datetime).transpose()?;
            let end = r.end.as_deref().map(parse_datetime).transpose()?;
            display(reader, start, end)
        }
        DisplayCommands::Tags => display_tags(reader),
        DisplayCommands::Projects => display_projects(reader),
    }
}

pub fn display(
    reader: &Reader,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
) -> anyhow::Result<()> {
    display_timeline(reader, start, end)
}

pub fn display_timeline(
    reader: &Reader,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
) -> anyhow::Result<()> {
    let start_ms = start.map(|dt| dt.timestamp_millis());
    let end_ms = end.map(|dt| dt.timestamp_millis());
    let records = reader.get_records(start_ms, end_ms)?;
    let tag_map: HashMap<String, (String, Option<String>)> =
        reader.get_tag_map().unwrap_or_default();
    for r in records {
        let raw_name = r.name.as_deref().unwrap_or("");
        if raw_name.trim().is_empty() {
            continue;
        }
        let name = raw_name;
        let start_str = DateTime::from_timestamp_millis(r.start)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| r.start.to_string());
        let end_str = r
            .end
            .and_then(DateTime::from_timestamp_millis)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| "-".to_string());
        let tags_str = match &r.tag_ids {
            Some(ids) if !ids.trim().is_empty() => {
                let parts: Vec<String> = ids
                    .split(',')
                    .filter_map(|id| {
                        let id = id.trim();
                        if id.is_empty() {
                            return None;
                        }
                        let (tag_name, color) = tag_map
                            .get(id)
                            .map(|(n, c)| (n.as_str(), c.as_deref()))
                            .unwrap_or((id, None));
                        Some(format_colored(tag_name, color))
                    })
                    .collect();
                if parts.is_empty() {
                    "-".to_string()
                } else {
                    parts.join(", ")
                }
            }
            _ => "-".to_string(),
        };
        let project_str = match &r.project_name {
            Some(pname) if !pname.trim().is_empty() => {
                format_colored(pname, r.project_color.as_deref())
            }
            _ => "-".to_string(),
        };
        println!("{name} | {start_str} | {end_str} | {tags_str} | {project_str}");
    }
    Ok(())
}

pub fn display_tags(reader: &Reader) -> anyhow::Result<()> {
    let tags = reader.get_tags()?;
    for tag in tags {
        if tag.name.trim().is_empty() {
            continue;
        }
        let colored = format_colored(&tag.name, tag.color.as_deref());
        println!("{colored}");
    }
    Ok(())
}

pub fn display_projects(reader: &Reader) -> anyhow::Result<()> {
    let projects = reader.get_projects()?;
    for proj in projects {
        if proj.name.trim().is_empty() {
            continue;
        }
        let colored = format_colored(&proj.name, proj.color.as_deref());
        println!("{colored}");
    }
    Ok(())
}

fn parse_datetime(s: &str) -> anyhow::Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(s)?.with_timezone(&Utc))
}

fn parse_colors(color_json: Option<&str>) -> Vec<owo_colors::Rgb> {
    let Some(json) = color_json else {
        return Vec::new();
    };
    let json = json.trim();
    if json.is_empty() || json == "null" || json == "[]" {
        return Vec::new();
    }
    serde_json::from_str::<Vec<Color>>(json)
        .map(|v| v.into_iter().map(|c| hex_to_rgb(c.0.hex(&Mode::Light))).collect())
        .unwrap_or_default()
}

fn format_colored(name: &str, color_json: Option<&str>) -> String {
    let colors = parse_colors(color_json);
    if colors.is_empty() {
        return name.to_string();
    }
    if colors.len() == 1 {
        return format!("{}", name.color(colors[0]));
    }
    let chars: Vec<char> = name.chars().collect();
    let n = chars.len();
    if n == 0 {
        return String::new();
    }
    let mut out = String::new();
    for (i, ch) in chars.into_iter().enumerate() {
        let idx = (i * colors.len() / n).min(colors.len() - 1);
        out.push_str(&format!("{}", ch.to_string().color(colors[idx])));
    }
    out
}
