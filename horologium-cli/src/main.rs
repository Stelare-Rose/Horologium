use clap::Parser;
use dirs::{cache_dir, data_dir};
use horologium_lib::{actions::Actions, compile::Compile, database::Database};

use crate::{cli::{Cli, Commands, TagCommands}, utils::{open_editor, resolve_color, resolve_project, resolve_tags}};

mod cli;
mod utils;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let base_path = cli.base_path.unwrap_or_else(|| data_dir().expect("Could not resolve data directory").join("Horologium"));
    let db_path = cli.db_path.unwrap_or_else(|| cache_dir().expect("Could not resolve cache directory").join("Horologium").join("main.db"));
    let database: Database = Database::new(&db_path)?;
    let actions: Actions = Actions::new(base_path.clone(), "test-id".to_string())?;
    let mut compile: Compile = Compile::new(base_path, Database::new(&db_path)?)?;
    match cli.command {
        Commands::Start(args) => {
            let tags = resolve_tags(args.tags, &database)?;
            let project = match args.project {
                Some(candidate) => Some(resolve_project(candidate, &database)?),
                None => None
            };
            let body = resolve_body(args.body)?; 
            
            let path = actions.start(args.name, tags, project, body, None)?;
            compile.compile_path(&path)?;
        }
        Commands::Stop(args) => {
            let body = resolve_body(args.body)?;           

            let path = actions.stop(args.name, body, None)?;
            compile.compile_path(&path)?;
        }
        Commands::Compile => {
            compile.compile_tags()?;
            compile.compile_projects()?;
            compile.compile_all_records()?;
            println!("Compilation success!");
        }
        Commands::Tag(args) => {
            match args.command {
                TagCommands::Define(args) => {
                    let colors = args.color
                        .iter()
                        .map(|c| resolve_color(c.to_string()))
                        .collect::<Result<Vec<_>, _>>()?;

                    let path = actions.define_tag(args.name, colors)?;
                    compile.compile_path(&path)?;
                }
            };
        }
    }
    Ok(())
}

fn resolve_body(body: Option<String>) -> anyhow::Result<Option<String>> {
    match body {
        Some(body) if body == "__OPEN_EDITOR__" => {
            Ok(Some(open_editor()?))
        }
        Some(body) => Ok(Some(body)),
        None => Ok(None)
    }
}
