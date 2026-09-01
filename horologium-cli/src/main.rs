use clap::Parser;
use horologium_lib::{actions::Actions, compile::Compile, database::Database, types::{Colorscheme, Mode}};
use owo_colors::OwoColorize;

use crate::{
    cli::{
        debug::DebugCommands,
        project::handle_project,
        record::{handle_start, handle_stop},
        tag::handle_tag,
        Cli, Commands,
    },
    utils::{hex_to_rgb, load_config},
};

mod cli;
mod utils;
mod reader;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = load_config()?;
    let base_path = cli.base_path.unwrap_or(config.base_path);
    let db_path = cli.db_path.unwrap_or(config.db_path);
    let database: Database = Database::new(&db_path)?;
    let actions: Actions = Actions::new(base_path.clone(), config.device_id)?;
    let mut compile: Compile = Compile::new(base_path, Database::new(&db_path)?)?;
    match cli.command {
        Commands::Start(args) => handle_start(args, &actions, &database)?,
        Commands::Stop(args) => handle_stop(args, &actions)?,
        Commands::Compile => {
            compile.compile_tags()?;
            compile.compile_projects()?;
            compile.compile_all_records()?;
            println!("Compilation success!");
        }
        Commands::Tag(args) => handle_tag(args, &actions)?,
        Commands::Project(args) => handle_project(args, &actions)?,
        Commands::Debug(args) => match args.command {
            DebugCommands::DisplayColors => {
                let colors = [
                    Colorscheme::Strawberry,
                    Colorscheme::Orange,
                    Colorscheme::Lemon,
                    Colorscheme::Leaf,
                    Colorscheme::Mint,
                    Colorscheme::Sky,
                    Colorscheme::Blueberry,
                    Colorscheme::Grape,
                    Colorscheme::Plum,
                    Colorscheme::Lavender,
                    Colorscheme::Lilac,
                    Colorscheme::Pink,
                ];
                let mode: Mode = Mode::Light;
                for color in colors {
                    let hex = color.hex(&mode);
                    let rgb = hex_to_rgb(hex);
                    println!("{}", format!("{}", color).color(rgb));
                }
            }
        },
    }
    Ok(())
}
