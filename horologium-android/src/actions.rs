use std::path::PathBuf;
use std::ptr::null_mut;

use chrono::{DateTime, Utc};
use horologium_lib::types::{ProjectId, TagId};
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

use crate::utils::{json, opt_json, opt_rfc3339, opt_string};

use horologium_lib::actions::Actions;

fn start_inner(
    env: &mut JNIEnv,
    base_path: &JString,
    device_id: &JString,
    name: &JString,
    tags: &JString,
    project: &JString,
    body: &JString,
    start_time: &JString,
) -> anyhow::Result<String> {
    let base_path: String = env.get_string(base_path)?.into();
    let device_id: String = env.get_string(device_id)?.into();
    let name: String = env.get_string(name)?.into();
    let tags: Vec<TagId> = json(env, tags)?;
    let project: Option<ProjectId> = opt_json(env, project)?;
    let body: Option<String> = opt_string(env, body)?;
    let start_time: Option<DateTime<Utc>> = opt_rfc3339(env, start_time)?;

    let actions = Actions::new(PathBuf::from(base_path), device_id)?;
    let id = actions.start(name, tags, project, body, start_time)?;
    Ok(id.0)
}

fn stop_inner(
    env: &mut JNIEnv,
    base_path: &JString,
    device_id: &JString,
    name: &JString,
) -> anyhow::Result<String> {
    let base_path: String = env.get_string(base_path)?.into();
    let device_id: String = env.get_string(device_id)?.into();
    let name: String = env.get_string(name)?.into();

    let actions = Actions::new(PathBuf::from(base_path), device_id)?;
    let id = actions.stop(Some(name), None, None)?;
    Ok(id.0)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_cc_starrytea_horologium_Native_actions_start<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    base_path: JString<'l>,
    device_id: JString<'l>,
    name: JString<'l>,
    tags: JString<'l>,
    project: JString<'l>,
    body: JString<'l>,
    start_time: JString<'l>,
) -> jstring {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        start_inner(
            &mut env,
            &base_path,
            &device_id,
            &name,
            &tags,
            &project,
            &body,
            &start_time,
        )
    }));

    match result {
        Ok(Ok(id)) => env
            .new_string(id)
            .map(|s| s.into_raw())
            .unwrap_or(null_mut()),
        Ok(Err(e)) => {
            let _ = env.throw_new("java/lang/RuntimeException", format!("{e:#}"));
            null_mut()
        }
        Err(_) => {
            let _ = env.throw_new("java/lang/RuntimeException", "Rust panic in start()");
            null_mut()
        }
    }
}
