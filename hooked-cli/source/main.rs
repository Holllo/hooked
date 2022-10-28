//! # Hooked
//!
//! > **Git hooks manager.**

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

use std::{
  fs::{set_permissions, write, Permissions},
  os::unix::fs::PermissionsExt,
  path::PathBuf,
};

use {
  clap::Parser,
  color_eyre::{eyre::eyre, install, Result},
  hooked_library::Config,
  tera::{Context, Tera},
};

use crate::cli::{Args, MainSubcommands};

/// The default template for all hooks.
pub const DEFAULT_TEMPLATE: &str = include_str!("templates/default.sh");

/// All supported hook types.
pub const HOOK_TYPES: &[&str] = &["pre-commit"];

mod cli;

fn main() -> Result<()> {
  install()?;

  let args = Args::parse();
  let config = Config::from_toml_file(args.config)?;

  let git_hooks_dir = PathBuf::from(".git/hooks/");
  if !git_hooks_dir.exists() {
    return Err(eyre!("The \".git/hooks/\" directory does not exist"));
  }

  match args.command {
    MainSubcommands::Install { overwrite } => {
      for hook_type in HOOK_TYPES {
        let mut context = Context::new();
        context.insert("hook_type", hook_type);

        let hook_path = git_hooks_dir.join(hook_type);
        if hook_path.exists() && !overwrite {
          println!(
            "{:?} exists, use --overwrite to replace the existing file",
            hook_path
          );
          continue;
        }

        write(
          &hook_path,
          Tera::one_off(DEFAULT_TEMPLATE, &context, false)?,
        )?;
        set_permissions(hook_path, Permissions::from_mode(0o775))?;
      }
    }
  }

  Ok(())
}