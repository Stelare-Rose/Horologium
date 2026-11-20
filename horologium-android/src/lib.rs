use std::path::PathBuf;
use std::ptr::null_mut;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

use horologium_lib::actions::Actions;

fn start_inner(
    env: &mut JNIEnv,
    base_path: &JString,
    device_id: &JString,
    name: &JString,
) -> anyhow::Result<String> {
    let base_path: String = env.get_string(base_path)?.into();
    let device_id: String = env.get_string(device_id)?.into();
    let name: String = env.get_string(name)?.into();

    let actions = Actions::new(PathBuf::from(base_path), device_id)?;
    let id = actions.start(name, vec![], None, None, None)?;
    Ok(id.0)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_cc_starrytea_horologium_Native_start<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    base_path: JString<'l>,
    device_id: JString<'l>,
    name: JString<'l>,
) -> jstring {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        start_inner(&mut env, &base_path, &device_id, &name)
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
