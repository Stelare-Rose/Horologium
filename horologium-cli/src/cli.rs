use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::cli::{debug::DebugArgs, project::ProjectArgs, record::{StartArgs, StopArgs}, tag::TagArgs};

pub mod debug;
pub mod project;
pub mod record;
pub mod tag;

#[derive(Parser)]
#[command(name = "horologium", about = "A time tracker")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true)]
    pub base_path: Option<PathBuf>,

    #[arg(long, global = true)]
    pub db_path: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    Start(StartArgs),
    Stop(StopArgs),
    Tag(TagArgs),
    Project(ProjectArgs),
    Debug(DebugArgs),
    Compile,
}
