use std::path::PathBuf;

use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "horologium", about = "A time tracker")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the Horologium Data Directory (default .local/share/Horologium)
    #[arg(long, global = true)]
    pub base_path: Option<PathBuf>,

    /// Path to the Horologium Database Directory (default .cache/Horologium)
    #[arg(long, global = true)]
    pub db_path: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start a new event
    Start {
        name: String,
        #[arg(short, long, value_delimiter=',')]
        tags: Vec<String>,
        #[arg(short, long)]
        project: Option<String>,
    },
    Stop {
        name: Option<String>,
    },
    Compile
}
