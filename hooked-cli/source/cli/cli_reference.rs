//! The `cli-reference` subcommand, only available in debug mode.

use std::{fs::write, process::Command, str};

use {
  color_eyre::Result,
  hooked_config::Config,
  tera::{Context, Tera},
};

use crate::cli::CliReferenceArgs;

const REFERENCE_TEMPLATE: &str = include_str!("../templates/cli-reference.txt");

/// The `cli-reference` subcommand.
pub fn hooked_cli_reference(
  _config: Config,
  args: CliReferenceArgs,
) -> Result<()> {
  let out_path = if args.output.is_dir() {
    args.output.join("cli-reference.txt")
  } else {
    args.output
  };

  let commands = {
    let mut commands = vec![];
    let commands_to_document = &["install", "run", "uninstall"];

    for command_name in commands_to_document {
      let output = Command::new("cargo")
        .env("NO_COLOR", "1")
        .args(&["run", "-q", "--", command_name, "--help"])
        .output()
        .unwrap();
      let usage = str::from_utf8(&output.stdout).unwrap().trim().to_string();
      commands.push((command_name.to_string(), usage))
    }

    commands
  };

  let mut context = Context::new();
  context.insert("commands", &commands);
  write(
    &out_path,
    Tera::one_off(REFERENCE_TEMPLATE, &context, false)?,
  )?;

  Ok(())
}
