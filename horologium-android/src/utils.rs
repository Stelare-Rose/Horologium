use chrono::{DateTime, Utc};
use jni::JNIEnv;
use jni::objects::JString;
use serde::de::DeserializeOwned;

/// Nullable Java string -> Option<String>
pub fn opt_string(env: &mut JNIEnv, s: &JString) -> anyhow::Result<Option<String>> {
    match s.is_null() {
        true => Ok(None),
        false => Ok(Some(env.get_string(s)?.into())),
    }
}

/// Non-null JSON string -> any deserializable type (e.g. Vec<TagId>)
pub fn json<T: DeserializeOwned>(env: &mut JNIEnv, s: &JString) -> anyhow::Result<T> {
    let raw: String = env.get_string(s)?.into();
    Ok(serde_json::from_str(&raw)?)
}

/// Nullable JSON string -> Option<T> (e.g. Option<ProjectId>)
pub fn opt_json<T: DeserializeOwned>(env: &mut JNIEnv, s: &JString) -> anyhow::Result<Option<T>> {
    Ok(opt_string(env, s)?
        .map(|raw| serde_json::from_str::<T>(&raw))
        .transpose()?)
}

/// Nullable RFC3339 timestamp -> Option<DateTime<Utc>>
pub fn opt_rfc3339(env: &mut JNIEnv, s: &JString) -> anyhow::Result<Option<DateTime<Utc>>> {
    match s.is_null() {
        true => Ok(None),
        false => {
            let raw: String = env.get_string(s)?.into();
            Ok(Some(
                DateTime::parse_from_rfc3339(&raw)?.with_timezone(&Utc),
            ))
        }
    }
}
