//! [`clap`] Derive structs.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
  Install {
    /// Overwrite existing files.
    #[clap(long)]
    overwrite: bool,
  },

  /// Remove installed hooks.
  Uninstall {
    /// Remove hooks not installed by Hooked.
    #[clap(long)]
    all: bool,
  },

  /// Manually run hooks.
  Run {
    /// The hook type to run.
    #[clap(value_parser = crate::HOOK_TYPES)]
    hook_type: String,
  },
}
