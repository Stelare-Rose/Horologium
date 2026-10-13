use std::collections::HashMap;

use anyhow::anyhow;
use constellation_eridanus::{Eri, Value};

use super::*;

fn max_secs(length: &QuotaRuleLength) -> i64 {
    match length {
        QuotaRuleLength::Day => 86_400,
        QuotaRuleLength::Week => 604_800,
        QuotaRuleLength::Month => 2_678_400,
    }
}

impl TryFrom<Eri> for Quotas {
    type Error = anyhow::Error;
    fn try_from(value: Eri) -> Result<Self, Self::Error> {
        let id = value.content.get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing id Value"))?;

        let name = value.content.get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let rule_object = value.content.get("rule")
            .and_then(|v| v.as_object())
            .ok_or_else(|| anyhow!("Missing rule object"))?;

        let length = match rule_object.get("length").and_then(|v| v.as_str()) {
            Some("day") => QuotaRuleLength::Day,
            Some("week") => QuotaRuleLength::Week,
            Some("month") => QuotaRuleLength::Month,
            Some(_) => return Err(anyhow!("Invalid rule.length for Quota")),
            None => return Err(anyhow!("Missing rule.length")),
        };

        let quota_type = match rule_object.get("type").and_then(|v| v.as_str()) {
            Some("max") => QuotaRuleType::Max,
            Some("min") => QuotaRuleType::Min,
            Some(_) => return Err(anyhow!("Invalid rule.type for Quota")),
            None => return Err(anyhow!("Missing rule.type")),
        };

        let secs = rule_object.get("secs")
            .and_then(|v| v.as_int())
            .ok_or_else(|| anyhow!("Missing rule.secs"))?;
        if secs <= 0 {
            return Err(anyhow!("rule.secs must be positive"));
        }
        if secs > max_secs(&length) {
            return Err(anyhow!("rule.secs exceeds the cap for this rule length"));
        }

        let state = match value.content.get("quota-state").and_then(|v| v.as_str()) {
            Some("active") => QuotaState::Active,
            Some("inactive") => QuotaState::Inactive,
            Some(_) => return Err(anyhow!("Invalid quota-state")),
            None => return Err(anyhow!("Missing quota-state")),
        };

        let target_object = value.content.get("target")
            .and_then(|v| v.as_object())
            .ok_or_else(|| anyhow!("Missing target object"))?;
        let (target_id, target_name) = target_object.get("id")
            .and_then(|v| v.as_id_tuple())
            .ok_or_else(|| anyhow!("Missing target.id"))?;
        let target = match target_object.get("enum").and_then(|v| v.as_str()) {
            Some("tag") => QuotaTarget::Tag {
                id: TagId { id: target_id.to_string(), name: target_name.to_string() },
            },
            Some("project") => QuotaTarget::Project {
                id: ProjectId { id: target_id.to_string(), name: target_name.to_string() },
            },
            Some(_) => return Err(anyhow!("Invalid target.enum")),
            None => return Err(anyhow!("Missing target.enum")),
        };

        Ok(Quotas {
            id: id.to_string(),
            name,
            rule: QuotaRule { length, quota_type, secs: secs as i32 },
            state,
            target,
        })
    }
}

impl From<Quotas> for Eri {
    fn from(value: Quotas) -> Self {
        const SCHEMA: &str = "horologium:quota";
        const VERSION: &str = "v1.0.0";
        let mut content: HashMap<String, Value> = HashMap::new();

        content.insert("id".to_string(), Value::Str(value.id));
        if let Some(name) = value.name {
            content.insert("name".to_string(), Value::Str(name));
        }

        let mut rule_object: HashMap<String, Value> = HashMap::new();
        rule_object.insert("length".to_string(), Value::Str(match value.rule.length {
            QuotaRuleLength::Day => "day",
            QuotaRuleLength::Week => "week",
            QuotaRuleLength::Month => "month",
        }.to_string()));
        rule_object.insert("type".to_string(), Value::Str(match value.rule.quota_type {
            QuotaRuleType::Max => "max",
            QuotaRuleType::Min => "min",
        }.to_string()));
        rule_object.insert("secs".to_string(), Value::Int(value.rule.secs as i64)); // rename to your integer variant
        content.insert("rule".to_string(), Value::Object(rule_object));

        content.insert("quota-state".to_string(), Value::Str(match value.state {
            QuotaState::Active => "active",
            QuotaState::Inactive => "inactive",
        }.to_string()));

        let mut target_object: HashMap<String, Value> = HashMap::new();
        match value.target {
            QuotaTarget::Tag { id } => {
                target_object.insert("enum".to_string(), Value::Str("tag".to_string()));
                target_object.insert("id".to_string(), Value::Id(id.id, id.name));
            }
            QuotaTarget::Project { id } => {
                target_object.insert("enum".to_string(), Value::Str("project".to_string()));
                target_object.insert("id".to_string(), Value::Id(id.id, id.name));
            }
        }
        content.insert("target".to_string(), Value::Object(target_object));

        Eri { schema: SCHEMA.to_string(), version: VERSION.to_string(), content, body: None}
    }
}
