use clap::{Args, Subcommand};
use horologium_lib::{actions::Actions, types::Mode};
use owo_colors::OwoColorize;

use crate::utils::{hex_to_rgb, resolve_color};

#[derive(Args)]
pub struct TagArgs {
    #[command(subcommand)]
    pub command: TagCommands,
}

#[derive(Subcommand)]
pub enum TagCommands {
    Define(DefineTagArgs),
}

#[derive(Args)]
pub struct DefineTagArgs {
    pub name: String,
    #[arg(short, long, value_delimiter=',')]
    pub color: Vec<String>,
}

pub fn handle_tag(args: TagArgs, actions: &Actions) -> anyhow::Result<()> {
    match args.command {
        TagCommands::Define(args) => {
            let colors = args
                .color
                .iter()
                .map(|c| resolve_color(c.to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            let display = colors.first().map(|c| hex_to_rgb(c.0.hex(&Mode::Light)));
            let tag_id = actions.define_tag(args.name.clone(), colors)?;
            match display {
                Some(display) => println!(
                    "Successfully made tag {} with id {}",
                    args.name.color(display),
                    tag_id.id
                ),
                None => println!("Successfully made tag {} with id {}", args.name, tag_id.id),
            }
        }
    }
    Ok(())
}
