use std::path::PathBuf;

use clap::Parser;
use horologium_lib::actions::Actions;

use crate::cli::{Cli, Commands};

mod cli;
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let base_path = cli.base_path.unwrap_or_else(|| PathBuf::from("/tmp/test"));
    match cli.command {
        Commands::Start { name, tags, project } => {
            let a: Actions = Actions::new(base_path, "test-id".to_string())?;
            let res = a.start(name, Vec::new(), None, None, None)?;
            println!("New event created with id {}", res.0);
        }
        Commands::Stop { name } => {
            let a: Actions = Actions::new(base_path, "test-id".to_string())?;
            let res = a.stop(name, None, None)?;
            println!("New event created with id {}", res.0);
        }
        Commands::Compile => {
            println!("Compile Received");
        }
    }
    Ok(())
}
