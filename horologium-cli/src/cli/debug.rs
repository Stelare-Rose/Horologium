use clap::{Subcommand, Args};
#[derive(Args)]
pub struct DebugArgs{
    #[command(subcommand)]
    pub command: DebugCommands
}
#[derive(Subcommand)]
pub enum DebugCommands {
    DisplayColors,
}
