use std::vec;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use constellation_eridanus::Eri;

pub struct EventId(String);
pub struct TagId(String);
pub struct ProjectId(String);
pub enum Event {
    Active {
        id: EventId,
        name: String,
        start: DateTime<Utc>,
        tags: Vec<TagId>,
        project: Option<ProjectId>,
        body: Option<String>
    },
    Inactive {
        id: EventId,
        name: Option<String>,
        start: DateTime<Utc>,
        body: Option<String>
    }
}


impl TryFrom<Eri> for Event {
    type Error = anyhow::Error;
    fn try_from(value: Eri) -> Result<Self, anyhow::Error> {
        let event_type = value.content.get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing Event Type"))?;
        match event_type {
            "active" => {
                let id = value.content.get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow!("Missing id Value"))?;
                let name = value.content.get("name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow!("Missing name Value"))?;
                let start_str = value.content.get("start")
                    .and_then(|v| v.as_date_str())
                    .ok_or_else(|| anyhow!("Missing start Time"))?;
                let start: DateTime<Utc> = DateTime::parse_from_rfc3339(start_str)?.with_timezone(&Utc);
                let tags: Vec<TagId> = value.content.get("tags")
                    .and_then(|v| v.as_array())
                    .map(|m| {
                        m.iter()
                        .filter_map(|i| i.as_id_ref())
                        .map(|j| TagId(j.to_string()))
                        .collect()
                    })
                    .unwrap_or_default();
                let project: Option<ProjectId> = value.content.get("project")
                    .and_then(|v| v.as_id_ref().map(|i| ProjectId(i.to_string())));
                Ok(Event::Active { id: EventId(id.to_string()), name: name.to_string(), start, tags, project, body: value.body })
            },
            "inactive" => {
                let id = value.content.get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow!("Missing id Value"))?;
                let name = value.content.get("name")
                    .map(|v| v.as_str())
                    .flatten();
                let start_str = value.content.get("start")
                    .and_then(|v| v.as_date_str())
                    .ok_or_else(|| anyhow!("Missing start Time"))?;
                let start: DateTime<Utc> = DateTime::parse_from_rfc3339(start_str)?.with_timezone(&Utc);
                Ok(Event::Inactive { id: EventId(id.to_string()), name: name.map(|n| n.to_string()), start, body: value.body })
            },
            _ => Err(anyhow!("Invalid Event Type!"))
        }
    }
}
