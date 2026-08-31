use clap::Args;
use horologium_lib::{actions::Actions, database::Database};

use crate::utils::{resolve_body, resolve_project, resolve_tags};

#[derive(Args)]
pub struct StartArgs {
    pub name: String,
    #[arg(short, long, value_delimiter=',')]
    pub tags: Vec<String>,
    #[arg(short, long)]
    pub project: Option<String>,
    #[arg(short, long, num_args = 0..=1, default_missing_value="__OPEN_EDITOR__")]
    pub body: Option<String>,
}

#[derive(Args)]
pub struct StopArgs {
    pub name: Option<String>,
    #[arg(short, long, num_args = 0..=1, default_missing_value="__OPEN_EDITOR__")]
    pub body: Option<String>,
}

pub fn handle_start(args: StartArgs, actions: &Actions, database: &Database) -> anyhow::Result<()> {
    let tags = resolve_tags(args.tags, database)?;
    let project = match args.project {
        Some(candidate) => Some(resolve_project(candidate, database)?),
        None => None,
    };
    let body = resolve_body(args.body)?;
    let name = args.name;
    let record_id = actions.start(name.clone(), tags, project, body, None)?;
    println!("Successfully made event {name} with id {}", record_id.0);
    Ok(())
}

pub fn handle_stop(args: StopArgs, actions: &Actions) -> anyhow::Result<()> {
    let body = resolve_body(args.body)?;
    let name = args.name;
    let record_id = actions.stop(name, body, None)?;
    println!("Successfully made stop event with id {}", record_id.0);
    Ok(())
}
