use horologium_lib::{
    compile::Compile,
    database::{self, Database},
};
use jni::{JNIEnv, objects::JString};

mod actions;
mod utils;
mod wrapper;

static INIT_LOCK: Mutex<()> = Mutex::new(());
static STATE: OnceLock<Mutex<Compile>> = OnceLock::new();

fn init_inner(env: &mut JNIEnv, base_path: &JString, db_path: &JString) -> anyhow::Result<()> {
    let _guard = INIT_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    if STATE.get().is_some() {
        return Ok(()); // Idempotency
    }
    let base_path: String = env.get_string(base_path)?.into();
    let db_path: String = env.get_string(db_path)?.into();
    let database = Database::new(PathBuf::from(&db_path))?;
    let compile = Compile::new(PathBuf::from(&base_path), database)?;
    let _ = STATE.set(Mutex::new(compile));
    Ok(())
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_cc_starrytea_horologium_Native_init<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    base_path: JString<'l>,
    db_path: JString<'l>,
) {
    run_jni(&mut env, "init", |env| {
        init_inner(env, &base_path, &db_path)
    })
}

pub fn with_state<T>(f: impl FnOnce(&mut Compile) -> anyhow::Result<T>) -> anyhow::Result<T> {
    let mutex = STATE
        .get()
        .ok_or_else(|| anyhow!("native state not initialized, call init first"))?;
    let mut guard = mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut guard)
}
