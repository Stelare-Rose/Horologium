use std::{collections::HashSet, path::PathBuf, time::Duration};
use anyhow::{Context, Result};
use horologium_lib::{compile::{self, Compile}, database::{self, Database}};
use notify::{EventKind, RecursiveMode, Watcher};
use tokio::{io::join, sync::mpsc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let watch_root: PathBuf = dirs::data_dir()
        .context("init | could not resolve data dir")?
        .join("Horologium"); // adjust to your actual layout
    let db_path: PathBuf = dirs::cache_dir()
        .context("init | could not resolve cache dir")?
        .join("Horologium")
        .join("main.db");
    reconcile_all(&db_path, &watch_root).context("init | startup compilation failed")?;

    let database: Database = Database::new(&db_path)?;
    let compile: Compile = Compile::new(watch_root.clone(), database)?;
    let (tx, mut rx) = mpsc::channel(256);
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res {
            let _ = tx.blocking_send(event);
        }
    })
        .context("init | failed to construct watcher")?;
    watcher
        .watch(&watch_root, RecursiveMode::Recursive)
        .with_context(|| format!("init | failed to watch {watch_root:?}"))?;

    tracing::info!(?watch_root, "init | watching for changes");
    let mut pending: HashSet<PathBuf> = HashSet::new();
    let mut drain_tick = tokio::time::interval(Duration::from_millis(300));
    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                if !EventKind::is_access(&event.kind) {
                    for path in event.paths {
                        if path.extension().and_then(|e| e.to_str()) != Some("eri") {
                            continue;
                        }
                        pending.insert(path);
                    }
                }
            }
                _ = drain_tick.tick() => {
                    if !pending.is_empty() {
                        let batch: Vec<PathBuf> = pending.drain().collect();
                        for path in batch {
                            tracing::debug!(?path, "detected event at");
                            if let Err(e) = compile.compile_path(&path) {
                                tracing::error!(?path, error = %e, "compiling | compile_path failed");
                            }
                        }
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    break;
                }
        }
    }
    Ok(())

}

fn reconcile_all(db_path: &PathBuf, watch_path: &PathBuf) -> anyhow::Result<()> {
    let database: Database = Database::new(db_path)?;
    let mut compile: Compile = Compile::new(watch_path.to_path_buf(), database)?;
    compile.compile_projects()?;
    compile.compile_tags()?;
    compile.compile_all_records()?;
    Ok(())
}
