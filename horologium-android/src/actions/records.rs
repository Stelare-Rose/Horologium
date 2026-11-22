use std::path::PathBuf;
use std::ptr::null_mut;

use chrono::{DateTime, Utc};
use horologium_lib::types::{ProjectId, Record, RecordId, TagId};
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

use crate::utils::{json, opt_json, opt_rfc3339, opt_string};
use crate::wrapper::{self, run_jni};

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
    body: &JString,
    start_time: &JString,
) -> anyhow::Result<String> {
    let base_path: String = env.get_string(base_path)?.into();
    let device_id: String = env.get_string(device_id)?.into();
    let name: Option<String> = opt_string(env, name)?;
    let body: Option<String> = opt_string(env, body)?;
    let start_time: Option<DateTime<Utc>> = opt_rfc3339(env, start_time)?;

    let actions = Actions::new(PathBuf::from(base_path), device_id)?;
    let id = actions.stop(name, body, start_time)?;
    Ok(id.0)
}

fn modify_record_inner(
    env: &mut JNIEnv,
    base_path: &JString,
    device_id: &JString,
    from: &JString,
    record: &JString,
) -> anyhow::Result<String> {
    let base_path: String = env.get_string(base_path)?.into();
    let device_id: String = env.get_string(device_id)?.into();
    let from: String = env.get_string(from)?.into();
    let record: Record = json(env, record)?;

    let actions = Actions::new(PathBuf::from(base_path), device_id)?;
    let id = actions.modify_record(&PathBuf::from(from), record)?;
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
    run_jni(&mut env, "start", |env| {
        start_inner(
            env,
            &base_path,
            &device_id,
            &name,
            &tags,
            &project,
            &body,
            &start_time,
        )
    })
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_cc_starrytea_horologium_Native_actions_stop<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    base_path: JString<'l>,
    device_id: JString<'l>,
    name: JString<'l>,
    body: JString<'l>,
    start_time: JString<'l>,
) -> jstring {
    run_jni(&mut env, "stop", |env| {
        stop_inner(env, &base_path, &device_id, &name, &body, &start_time)
    })
}
#[unsafe(no_mangle)]
pub extern "system" fn Java_cc_starrytea_horologium_Native_actions_modify_record<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    base_path: JString<'l>,
    device_id: JString<'l>,
    from: JString<'l>,
    record: JString<'l>,
) -> jstring {
    run_jni(&mut env, "stop", |env| {
        modify_record_inner(env, &base_path, &device_id, &from, &record)
    })
}
