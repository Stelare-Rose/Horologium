use clap::{Args, Subcommand};
use horologium_lib::{actions::Actions, types::Mode};
use owo_colors::OwoColorize;

use crate::utils::{hex_to_rgb, resolve_body, resolve_color};

#[derive(Args)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: ProjectCommands,
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    Define(DefineProjectArgs),
}

#[derive(Args)]
pub struct DefineProjectArgs {
    pub name: String,
    #[arg(short, long, value_delimiter=',')]
    pub color: Vec<String>,
    #[arg(short, long, num_args = 0..=1, default_missing_value="__OPEN_EDITOR__")]
    pub body: Option<String>,
}

pub fn handle_project(args: ProjectArgs, actions: &Actions) -> anyhow::Result<()> {
    match args.command {
        ProjectCommands::Define(args) => {
            let colors = args
                .color
                .iter()
                .map(|c| resolve_color(c.to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            let display = colors.first().map(|c| hex_to_rgb(c.0.hex(&Mode::Light)));
            let body = resolve_body(args.body)?;
            let project_id = actions.define_project(args.name.clone(), colors, body)?;
            match display {
                Some(display) => println!(
                    "Successfully made project {} with id {}",
                    args.name.color(display),
                    project_id.id
                ),
                None => println!(
                    "Successfully made project {} with id {}",
                    args.name, project_id.id
                ),
            }
        }
    }
    Ok(())
}
