use super::*;
use std::collections::HashMap;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use constellation_eridanus::{Eri, Value};


impl TryFrom<Eri> for Record {
    type Error = anyhow::Error;
    fn try_from(value: Eri) -> Result<Self, anyhow::Error> {
        let id = value.content.get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing ID Value"))?;
        let start_str = value.content.get("start")
            .and_then(|v| v.as_date_str())
            .ok_or_else(|| anyhow!("Missing Start Time"))?;
        let start: DateTime<Utc> = DateTime::parse_from_rfc3339(start_str)?.with_timezone(&Utc);
        let event_object = value.content.get("event")
            .and_then(|v| v.as_object())
            .ok_or_else(|| anyhow!("Missing Event Object"))?;
        let event_type = event_object.get("enum")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing Event Type"))?;
        let event = match event_type {
            "active" => {
                let name = event_object.get("name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow!("Missing name Value"))?;
                let tags: Vec<TagId> = event_object.get("tags")
                    .and_then(|v| v.as_array())
                    .map(|m| {
                        m.iter()
                        .filter_map(|i| i.as_id_tuple())
                        .map(|j| TagId{id: j.0.to_string(), name: j.1.to_string()})
                        .collect()
                    })
                    .unwrap_or_default();
                let project: Option<ProjectId> = event_object.get("project")
                    .and_then(|v| v.as_id_tuple()
                        .map(|i| ProjectId{id: i.0.to_string(), name: i.1.to_string()})
                    );
                Event::Active { name: name.to_string(), tags, project }
            },
            "inactive" => {
                let name = event_object.get("name")
                    .and_then(|v| v.as_str());
                Event::Inactive { name: name.map(|n| n.to_string()) }
            },
            _ => return Err(anyhow!("Invalid Event Type!"))
        };
        Ok(Record{ id: RecordId(id.to_string()), start, event, body: value.body })
    }
}

impl From<Record> for Eri {
    fn from(value: Record) -> Self {
        const SCHEMA: &str = "horologium:event";
        const VERSION: &str = "v1.0.0";
        let mut content: HashMap<String, Value> = HashMap::new();
        let mut event: HashMap<String, Value> = HashMap::new();
        
        let (id, start, body) = (value.id.0, value.start, value.body);


        content.insert("id".to_string(), Value::Str(id));
        content.insert("start".to_string(), Value::DateTime(start.to_rfc3339()));
        
        match value.event {
            Event::Active { name, tags, project } => { 
                event.insert("enum".to_string(), Value::Str("active".to_string()));
                event.insert("tags".to_string(), Value::Array(
                    tags.iter()
                        .map(|v| {
                            Value::Id(v.id.clone(), v.name.clone())
                    }).collect()
                ));

                event.insert("name".to_string(), Value::Str(name));
                if let Some(p) = project {
                    event.insert("project".to_string(), Value::Id(p.id, p.name));
                }
            }
            Event::Inactive { name } => {
                event.insert("enum".to_string(), Value::Str("inactive".to_string()));
                if let Some(n) = name {
                    event.insert("name".to_string(), Value::Str(n));
                }
            }
        }
        content.insert("event".to_string(), Value::Object(event));

        Eri { schema: SCHEMA.to_string(), version: VERSION.to_string(), content, body }
    }
}
