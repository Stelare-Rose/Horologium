use std::collections::HashMap;

use anyhow::anyhow;
use constellation_eridanus::{Eri, Value};

use super::*;

impl TryFrom<Eri> for Tag {
    type Error = anyhow::Error;
    fn try_from(value: Eri) -> Result<Self, Self::Error> {
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
        Ok(Tag { id: id.to_string(), name: name.to_string(), color })
    }
}

impl From<Tag> for Eri {
    fn from(value: Tag) -> Self {
        const SCHEMA: &str = "horologium:tag";
        const VERSION: &str = "v1.0.0";
        let mut content: HashMap<String, Value> = HashMap::new();

        let (id, name, color) = (value.id, value.name, value.color);

        content.insert("id".to_string(), Value::Str(id));
        content.insert("name".to_string(), Value::Str(name));
        content.insert("color".to_string(), Value::Array(
            color.into_iter().map(|c| Value::Str(c.0)).collect()
        ));

        Eri { schema: SCHEMA.to_string(), version: VERSION.to_string(), content, body: None }
    }
    
}
