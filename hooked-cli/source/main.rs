//! # Hooked
//!
//! > **Git hooks manager.**

use {
  clap::Parser,
  color_eyre::{install, Result},
  hooked_config::Config,
};

use crate::cli::{Args, MainSubcommands};

/// The default template for all hooks.
pub const DEFAULT_TEMPLATE: &str = include_str!("templates/default.sh");

/// All supported hook types.
pub const HOOK_TYPES: [&str; 1] = ["pre-commit"];

mod cli;
mod printer;
mod utilities;

fn main() -> Result<()> {
  install()?;

  let args = Args::parse();
  let config = Config::from_toml_file(args.config)?;

  match args.command {
    MainSubcommands::Install(sub_args) => {
      cli::hooked_install(config, sub_args)?;
    }

    MainSubcommands::Uninstall(sub_args) => {
      cli::hooked_uninstall(config, sub_args)?;
    }

    MainSubcommands::Run(sub_args) => {
      cli::hooked_run(config, sub_args)?;
    }

    #[cfg(debug_assertions)]
    MainSubcommands::CliReference(sub_args) => {
      cli::hooked_cli_reference(config, sub_args)?;
    }
  }

  Ok(())
}
