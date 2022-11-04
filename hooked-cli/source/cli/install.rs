//! The `install` subcommand.
use std::{
  fs::{set_permissions, write, Permissions},
  os::unix::fs::PermissionsExt,
  path::PathBuf,
};

use {
  color_eyre::{eyre::eyre, Result},
  hooked_config::Config,
  tera::{Context, Tera},
};

use crate::{cli::InstallArgs, DEFAULT_TEMPLATE, HOOK_TYPES};

/// The `install` subcommand.
pub fn hooked_install(config: Config, args: InstallArgs) -> Result<()> {
  let git_hooks_dir = PathBuf::from(".git/hooks/");
  if !git_hooks_dir.exists() {
    return Err(eyre!("The \".git/hooks/\" directory does not exist"));
  }

  for hook_type in HOOK_TYPES {
    let mut context = Context::new();
    context.insert("config_path", &config.general.config);
    context.insert("hook_type", hook_type);

    let hook_path = git_hooks_dir.join(hook_type);
    if hook_path.exists() && !args.overwrite {
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

  Ok(())
}
