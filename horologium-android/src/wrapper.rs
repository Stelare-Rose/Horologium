use std::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr::null_mut;

use jni::JNIEnv;
use jni::sys::jstring;

pub trait IntoJni {
    type Jni;
    fn into_jni(self, env: &mut JNIEnv) -> anyhow::Result<Self::Jni>;
    fn failure() -> Self::Jni;
}

impl IntoJni for String {
    type Jni = jstring;
    fn into_jni(self, env: &mut JNIEnv) -> anyhow::Result<jstring> {
        Ok(env.new_string(self)?.into_raw())
    }
    fn failure() -> jstring {
        null_mut()
    }
}

impl IntoJni for () {
    type Jni = ();
    fn into_jni(self, _env: &mut JNIEnv) -> anyhow::Result<()> {
        Ok(())
    }
    fn failure() {}
}

fn panic_message(payload: &Box<dyn Any + Send>) -> &str {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s
    } else {
        "unknown panic"
    }
}

pub fn run_jni<T, F>(env: &mut JNIEnv, label: &str, f: F) -> T::Jni
where
    T: IntoJni,
    F: FnOnce(&mut JNIEnv) -> anyhow::Result<T>,
{
    let result = catch_unwind(AssertUnwindSafe(|| {
        let value = f(env)?;
        value.into_jni(env)
    }));

    match result {
        Ok(Ok(jni_value)) => jni_value,
        Ok(Err(e)) => {
            let _ = env.throw_new("java/lang/RuntimeException", format!("{label}: {e:#}"));
            T::failure()
        }
        Err(payload) => {
            let msg = format!("Rust panic in {label}: {}", panic_message(&payload));
            let _ = env.throw_new("java/lang/RuntimeException", msg);
            T::failure()
        }
    }
}
