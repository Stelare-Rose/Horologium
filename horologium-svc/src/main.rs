use std::{collections::HashSet, path::PathBuf, time::{Duration, Instant}};
use anyhow::{Context};
use horologium_lib::{compile::{Compile}, database::{Database}};
use notify::{EventKind, RecursiveMode, Watcher};
use tokio::{ sync::mpsc, time::interval};
use tracing::{info, error, debug};

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

    info!(?watch_root, "init | watching for changes");
    let mut pending: HashSet<PathBuf> = HashSet::new();
    let mut drain_tick = interval(Duration::from_millis(200));
    let mut needs_reconciliation: bool = false;
    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                debug!(?event, "file watcher | found event");
                if !EventKind::is_access(&event.kind) {
                    for path in &event.paths {
                        if path.extension().and_then(|e| e.to_str()) == Some("eri") {
                            pending.insert(path.clone());
                        } else if path.extension().is_none() && !event.kind.is_create() {
                            debug!("file watcher | reconciling..");
                            needs_reconciliation = true;
                        }
                    }
                }
            },
                _ = drain_tick.tick() => {
                    if needs_reconciliation {
                        info!("compiling | compile (cascade)");
                        reconcile_all(&db_path, &watch_root)?;
                        needs_reconciliation = false;
                        pending.clear();
                    } else if !pending.is_empty() {
                        let batch: Vec<PathBuf> = pending.drain().collect();
                        for path in batch {
                            debug!(?path, "detected event at");
                            let start = Instant::now();
                            if let Err(e) = compile.compile_path(&path) {
                                error!(?path, error = %e, "compiling | compile_path failed");
                            } else {
                                let elapsed = start.elapsed();
                                info!(item = %path.file_name().unwrap_or_default().to_string_lossy(), ?elapsed, "compiling | compile (single path) finished");
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
    let start = Instant::now();
    compile.compile_projects()?;
    compile.compile_tags()?;
    compile.compile_all_records()?;
    let elapsed = start.elapsed();
    info!(?elapsed, "init | compile completed");
    Ok(())
}
