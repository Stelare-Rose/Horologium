use std::collections::HashMap;

use anyhow::anyhow;
use constellation_eridanus::{Eri, Value};

use super::*;

impl TryFrom<Eri> for Project {
    type Error = anyhow::Error;
    fn try_from(value: Eri) -> Result<Self, Self::Error> {
        let state_object = value.content.get("project-state")
            .and_then(|v| v.as_object())
            .ok_or_else(|| anyhow!("Missing state object"))?;

        let state = state_object.get("enum")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing state enum"))?;

        let proj_state = match state {
            "active" => {
                ProjectState::Active
            },
            "done" => {
                let completed_string = state_object.get("completed-date")
                    .and_then(|v| v.as_date_str())
                    .ok_or_else(|| anyhow!("Missing completed Date for Completed Project"))?;
                let completed_date: NaiveDate = NaiveDate::parse_from_str(completed_string, "%Y-%m-%d")?;
                ProjectState::Done { completed_date: completed_date }
            }
            _ => {
                return Err(anyhow!("Invalid State for Project"));
            }
        };
        let id = value.content.get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing id Value"))?;
        let name = value.content.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing Name"))?;
        let color: Vec<Color> = value.content.get("color")
            .and_then(|v| v.as_array())
            .map(|m| { 
                m.iter()
                    .filter_map(|i| i.as_str())
                    .map(|s| Color(s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(Project { id: id.to_string(), name: name.to_string(), state: proj_state, color, body: value.body })
    }
}

impl From<Project> for Eri {
    fn from(value: Project) -> Self {
        const SCHEMA: &str = "horologium:project";
        const VERSION: &str = "v1.0.0";
        let mut content: HashMap<String, Value> = HashMap::new();

        let (id, name, color) = (value.id, value.name, value.color);

        match value.state {
            ProjectState::Active => {
                let mut state_object: HashMap<String, Value> = HashMap::new();
                
                state_object.insert("enum".to_string(), Value::Str("active".to_string()));
                content.insert("project-state".to_string(), Value::Object(state_object));
            }

            ProjectState::Done { completed_date } => {
                let mut state_object: HashMap<String, Value> = HashMap::new();
                state_object.insert("enum".to_string(), Value::Str("done".to_string()));
                state_object.insert("completed-date".to_string(), Value::Date(completed_date.format("%Y-%m-%d").to_string()));
                content.insert("project-state".to_string(), Value::Object(state_object));
            }
        }

        content.insert("id".to_string(), Value::Str(id));
        content.insert("name".to_string(), Value::Str(name));
        content.insert("color".to_string(), Value::Array(
            color.into_iter().map(|c| Value::Str(c.0)).collect()
        ));

        Eri { schema: SCHEMA.to_string(), version: VERSION.to_string(), content, body: value.body }
    }
    
}
