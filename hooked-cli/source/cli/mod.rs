//! [`clap`] Derive structs.

use std::path::PathBuf;

use clap::{Args as Arguments, Parser, Subcommand};

mod run;

pub use run::hooked_run;

/// CLI arguments struct using [`clap::Parser`].
#[derive(Debug, Parser)]
#[clap(about, author, version)]
#[clap(propagate_version = true)]
pub struct Args {
  /// The CLI subcommand.
  #[clap(subcommand)]
  pub command: MainSubcommands,

  /// Path to a Hooked configuration.
  #[clap(short, long, global = true, default_value = "Hooked.toml")]
  pub config: PathBuf,
}

/// Main CLI subcommands.
#[derive(Debug, Subcommand)]
pub enum MainSubcommands {
  /// Install Hooked into ".git/hooks".
  Install(InstallArgs),

  /// Remove installed hooks.
  Uninstall(UninstallArgs),

  /// Manually run hooks.
  Run(RunArgs),
}

/// The `install` subcommand arguments.
#[derive(Debug, Arguments)]
pub struct InstallArgs {
  /// Overwrite existing files.
  #[clap(long)]
  pub overwrite: bool,
}

/// The `uninstall` subcommand arguments.
#[derive(Debug, Arguments)]
pub struct UninstallArgs {
  /// Remove hooks not installed by Hooked.
  #[clap(long)]
  pub all: bool,
}

/// The `run` subcommand arguments.
#[derive(Debug, Arguments)]
pub struct RunArgs {
  /// The hook type to run.
  #[clap(value_parser = crate::HOOK_TYPES)]
  pub hook_type: String,
}
