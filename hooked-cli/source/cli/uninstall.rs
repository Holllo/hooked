//! The `uninstall` subcommand.

use std::{
  fs::{read_to_string, remove_file},
  path::PathBuf,
};

use {
  color_eyre::{eyre::eyre, Result},
  hooked_config::Config,
};

use crate::{cli::UninstallArgs, HOOK_TYPES};

/// The `uninstall` subcommand.
pub fn hooked_uninstall(_config: Config, args: UninstallArgs) -> Result<()> {
  let git_hooks_dir = PathBuf::from(".git/hooks/");
  if !git_hooks_dir.exists() {
    return Err(eyre!("The \".git/hooks/\" directory does not exist"));
  }

  for hook_type in HOOK_TYPES {
    let hook_path = git_hooks_dir.join(hook_type);
    if !hook_path.exists() {
      continue;
    }

    let hook_contents = read_to_string(&hook_path)?;
    if args.all || hook_contents.contains("# Installed by Hooked.") {
      remove_file(hook_path)?;
    } else {
      println!(
        "{:?} wasn't installed by Hooked, use --all to remove it",
        hook_path
      );
    }
  }

  Ok(())
}
