//! The `install` subcommand.
use std::{
  fs::{read_to_string, set_permissions, write, Permissions},
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
  let silent = args.silent;
  let git_hooks_dir = PathBuf::from(".git/hooks/");
  if !git_hooks_dir.exists() {
    return Err(eyre!("The \".git/hooks/\" directory does not exist"));
  }

  let hooked_directory = config.general.directory;
  for hook_type in HOOK_TYPES {
    let mut context = Context::new();
    context.insert("config_path", &config.general.config);
    context.insert("hook_type", hook_type);

    let hook_path = git_hooks_dir.join(hook_type);
    if hook_path.exists() && !args.overwrite {
      if !silent {
        println!(
          "{:?} exists, use --overwrite to replace the existing file",
          hook_path
        );
      }
      continue;
    }

    let template = match config.general.template.as_ref() {
      Some(template_path) => {
        read_to_string(hooked_directory.join(template_path))?
      }
      None => DEFAULT_TEMPLATE.to_string(),
    };

    write(&hook_path, Tera::one_off(&template, &context, false)?)?;
    set_permissions(hook_path, Permissions::from_mode(0o775))?;
  }

  Ok(())
}
