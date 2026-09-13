use std::collections::HashMap;

use chrono::{DateTime, Local, TimeZone, Utc};
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
    #[arg(long, help = "Show records from today (local timezone)")]
    pub today: bool,
    #[arg(value_name = "FILTER", help = "Optional filter, e.g. 'today'")]
    pub filter: Option<String>,
}

pub fn handle_display(args: DisplayArgs, reader: &Reader) -> anyhow::Result<()> {
    match args.command {
        DisplayCommands::Records(r) => {
            let filter_is_today = r.filter.as_deref() == Some("today");
            let is_today = r.today || filter_is_today;
            if r.today && r.filter.is_some() && !filter_is_today {
                anyhow::bail!("unknown filter '{}', expected 'today'", r.filter.unwrap());
            }
            if is_today {
                if r.start.is_some() || r.end.is_some() {
                    anyhow::bail!("--start/--end cannot be used with 'today'");
                }
                let (start, end) = today_bounds();
                display(reader, Some(start), Some(end))
            } else {
                if let Some(f) = r.filter {
                    anyhow::bail!("unknown filter '{f}', expected 'today'");
                }
                let start = r.start.as_deref().map(parse_datetime).transpose()?;
                let end = r.end.as_deref().map(parse_datetime).transpose()?;
                display(reader, start, end)
            }
        }
        DisplayCommands::Tags => display_tags(reader),
        DisplayCommands::Projects => display_projects(reader),
    }
}

fn today_bounds() -> (DateTime<Utc>, DateTime<Utc>) {
    let today = Local::now().date_naive();
    let start_naive = today.and_hms_opt(0, 0, 0).unwrap();
    let next_day = today.checked_add_days(chrono::Days::new(1)).unwrap();
    let next_start_naive = next_day.and_hms_opt(0, 0, 0).unwrap();
    let resolve = |naive: chrono::NaiveDateTime| match naive.and_local_timezone(Local) {
        chrono::MappedLocalTime::Single(dt) => dt,
        chrono::MappedLocalTime::Ambiguous(a, _) => a,
        chrono::MappedLocalTime::None => Local.from_local_datetime(&naive).earliest().unwrap(),
    };
    let start_local = resolve(start_naive);
    let next_start_local = resolve(next_start_naive);
    let start_utc = start_local.with_timezone(&Utc);
    let end_utc = next_start_local.with_timezone(&Utc) - chrono::TimeDelta::milliseconds(1);
    (start_utc, end_utc)
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
    let mut rows: Vec<Vec<String>> = Vec::new();
    for r in records {
        let mut raw_name = r.name.as_deref().unwrap_or("");
        if raw_name.trim().is_empty() {
            raw_name = "Untracked";
        }
        let name = raw_name.to_string();
        let start_str = DateTime::from_timestamp_millis(r.start)
            .map(|dt| dt.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| r.start.to_string());
        let end_str = r
            .end
            .and_then(DateTime::from_timestamp_millis)
            .map(|dt| dt.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "-".to_string());
        let duration_str = {
            let end_ms = r.end.unwrap_or_else(|| Utc::now().timestamp_millis());
            format_duration(r.start, end_ms)
        };
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
        rows.push(vec![name, start_str, end_str, duration_str, tags_str, project_str]);
    }
    print_table(
        &["Name", "Start", "End", "Duration", "Tags", "Projects"],
        rows,
        Some("Records"),
    );
    Ok(())
}

pub fn display_tags(reader: &Reader) -> anyhow::Result<()> {
    let tags = reader.get_tags()?;
    let mut rows: Vec<Vec<String>> = Vec::new();
    for tag in tags {
        if tag.name.trim().is_empty() {
            continue;
        }
        let colored = format_colored(&tag.name, tag.color.as_deref());
        rows.push(vec![colored]);
    }
    print_table(&["Tag"], rows, Some("Tags"));
    Ok(())
}

pub fn display_projects(reader: &Reader) -> anyhow::Result<()> {
    let projects = reader.get_projects()?;
    let mut rows: Vec<Vec<String>> = Vec::new();
    for proj in projects {
        if proj.name.trim().is_empty() {
            continue;
        }
        let colored = format_colored(&proj.name, proj.color.as_deref());
        rows.push(vec![colored]);
    }
    print_table(&["Project"], rows, Some("Projects"));
    Ok(())
}

fn format_duration(start_ms: i64, end_ms: i64) -> String {
    let diff = end_ms - start_ms;
    if diff < 0 {
        return "-".to_string();
    }
    let total_secs = diff / 1000;
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    if days > 0 {
        format!("{days}d {hours}h {minutes}m {seconds}s")
    } else if hours > 0 {
        format!("{hours}h {minutes}m {seconds}s")
    } else if minutes > 0 {
        format!("{minutes}m {seconds}s")
    } else {
        format!("{seconds}s")
    }
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

fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next == 'm' {
                        break;
                    }
                }
                continue;
            }
        }
        len += 1;
    }
    len
}

fn pad_cell(s: &str, width: usize) -> String {
    let vlen = visible_len(s);
    if vlen >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - vlen))
    }
}

fn print_table(headers: &[&str], rows: Vec<Vec<String>>, title: Option<&str>) {
    if let Some(t) = title {
        println!("{t}");
    }
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            let vlen = visible_len(cell);
            if i < widths.len() {
                widths[i] = widths[i].max(vlen);
            } else {
                widths.push(vlen);
            }
        }
    }
    let border = {
        let mut s = String::from("+");
        for w in &widths {
            s.push_str(&"-".repeat(w + 2));
            s.push('+');
        }
        s
    };
    let header_row = {
        let mut s = String::from("|");
        for (i, h) in headers.iter().enumerate() {
            s.push(' ');
            s.push_str(&pad_cell(h, widths[i]));
            s.push_str(" |");
        }
        s
    };
    println!("{border}");
    println!("{header_row}");
    println!("{border}");
    if rows.is_empty() {
        let empty = {
            let total_width = widths.iter().map(|w| w + 3).sum::<usize>() - 1;
            let msg = "(no entries)";
            let pad = total_width.saturating_sub(msg.len());
            let left = pad / 2;
            let right = pad - left;
            format!("|{}{}{}|", " ".repeat(left), msg, " ".repeat(right))
        };
        println!("{empty}");
    } else {
        for row in rows {
            let mut s = String::from("|");
            for (i, cell) in row.iter().enumerate() {
                s.push(' ');
                s.push_str(&pad_cell(cell, widths[i]));
                s.push_str(" |");
            }
            for i in row.len()..widths.len() {
                s.push(' ');
                s.push_str(&pad_cell("", widths[i]));
                s.push_str(" |");
            }
            println!("{s}");
        }
    }
    println!("{border}");
}
