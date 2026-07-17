use std::collections::HashMap;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use constellation_eridanus::{Eri, Value};

pub struct EventId(String);
pub struct TagId(String, String);   // (id, name)
pub struct ProjectId(String, String); // (id, name)
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
                        .filter_map(|i| i.as_id_tuple())
                        .map(|j| TagId(j.0.to_string(), j.1.to_string()))
                        .collect()
                    })
                    .unwrap_or_default();
                let project: Option<ProjectId> = value.content.get("project")
                    .and_then(|v| v.as_id_tuple().map(|i| ProjectId(i.0.to_string(), i.1.to_string())));
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

impl From<Event> for Eri {
    fn from(value: Event) -> Self {
        const SCHEMA: &str = "horologium:event";
        const VERSION: &str = "v1.0.0";
        let mut content: HashMap<String, Value> = HashMap::new();
        
        let (id, start, body, event_type) = match &value {
            Event::Active { id, start, body, .. } => (id.0.clone(), *start, body.clone(), "active"),
            Event::Inactive { id, start, body, .. } => (id.0.clone(), *start, body.clone(), "inactive"),
        };

        content.insert("type".to_string(), Value::Str(event_type.to_string()));
        content.insert("id".to_string(), Value::Str(id));
        content.insert("start".to_string(), Value::DateTime(start.to_rfc3339()));
        match value {
            Event::Active { name, tags, project, .. } => {
                content.insert("name".to_string(), Value::Str(name));
                if !tags.is_empty() {
                    content.insert("tags".to_string(), Value::Array(
                        tags.into_iter().map(|t| Value::Id(t.0, t.1)).collect()
                    ));
                }
                if let Some(p) = project {
                    content.insert("project".to_string(), Value::Id(p.0, p.1));
                }
            }
            Event::Inactive { name, .. } => {
                if let Some(n) = name {
                    content.insert("name".to_string(), Value::Str(n));
                }
            }
        }

        Eri { schema: SCHEMA.to_string(), version: VERSION.to_string(), content, body }
    }
}
