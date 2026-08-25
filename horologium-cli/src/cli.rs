use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};


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
    Start(StartArgs),
    Stop(StopArgs),
    Tag(TagArgs),
    Compile
}

#[derive(Args)]
pub struct StartArgs {
    /// The name of the new event
    pub name: String,
    /// Optional list of tags for the event, delimited by ","
    #[arg(short, long, value_delimiter=',')]
    pub tags: Vec<String>,
    /// Optional project that the event belongs to
    #[arg(short, long)]
    pub project: Option<String>,
    /// Optional body text, pass nothing to edit in terminal editor
    #[arg(short, long, num_args = 0..=1, default_missing_value="__OPEN_EDITOR__")]
    pub body: Option<String>,
}
#[derive(Args)]
pub struct StopArgs {
    /// The name of the new event
    pub name: Option<String>,
}
#[derive(Args)]
pub struct TagArgs {
    #[command(subcommand)]
    pub command: TagCommands
}
#[derive(Subcommand)]
pub enum TagCommands {
    Define(DefineArgs)
}
#[derive(Args)]
pub struct DefineArgs {
    /// The name of the new tag
    pub name: String,
    /// Optional list of colors, delimited by "," Available colors are
    /// strawberry, orange, lemon, leaf, mint, sky, blueberry, grape, plum, lavender, lilac, and pink.
    pub color: Vec<String>
}
