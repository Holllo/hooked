//! The `run` subcommand.

use std::{io::Read, process::exit};

use {
  color_eyre::{eyre::eyre, Result},
  hooked_config::{Config, ExitAction},
  owo_colors::OwoColorize,
  subprocess::{Exec, Redirection},
};

use crate::{
  cli::RunArgs,
  printer::{print, PrintType, PRINT_STYLE},
  utilities::{globset_from_strings, plural},
};

/// The `run` subcommand.
pub fn hooked_run(config: Config, args: RunArgs) -> Result<()> {
  let cli_noise_level = args.noise_level.as_ref();
  let global_noise_level = &config.general.noise_level;

  if args.hook_type == "pre-commit" {
    let hook_count = config.pre_commit.len();
    print(
      format!(
        "Hooked: Running {} pre-commit {}.",
        hook_count,
        plural(hook_count, "hook", None)
      ),
      cli_noise_level.unwrap_or(global_noise_level),
      PrintType::Info,
    );

    'hook_loop: for hook in config.pre_commit {
      let hook_name = hook.name.unwrap_or_else(|| "Unnamed Hook".to_string());

      if !hook.staged.is_empty() {
        let globs = globset_from_strings(&hook.staged)?;

        let staged_files = Exec::cmd("git")
          .args(&["diff", "--name-only", "--cached"])
          .capture()?
          .stdout_str();
        if !staged_files.lines().any(|line| globs.is_match(line)) {
          print(
            format!(
              "\t{} {}",
              "≫".style(PRINT_STYLE.skipped),
              hook_name.style(PRINT_STYLE.skipped)
            ),
            cli_noise_level.unwrap_or(global_noise_level),
            PrintType::Info,
          );
          continue 'hook_loop;
        }
      }

      let command = match (hook.task.command, hook.task.script) {
        (Some(command), _) => Ok(Exec::shell(command)),

        (None, Some(script_file)) => {
          let script_path = config.general.directory.join(script_file);
          let script_path_str = script_path
            .to_str()
            .ok_or_else(|| eyre!("Failed to convert path to str"))?;
          Ok(Exec::shell(script_path_str))
        }

        (None, None) => Err(eyre!(
          "No command or script provided for hook: {}",
          hook_name
        )),
      }?;

      let mut process = command
        .stderr(Redirection::Merge)
        .stdout(Redirection::Pipe)
        .popen()?;
      let exit_status = process.wait()?;
      let output = {
        let mut output = String::new();
        let mut stdout_file = process.stdout.take().unwrap();
        stdout_file.read_to_string(&mut output)?;
        output
      };

      let (stop, print_output, prefix, style, print_type) =
        match (exit_status.success(), hook.on_failure) {
          (true, _) => {
            (false, false, "✓", PRINT_STYLE.success, PrintType::Info)
          }
          (false, ExitAction::Continue) => {
            (false, true, "⚠", PRINT_STYLE.warn, PrintType::Warn)
          }
          (false, ExitAction::Stop) => {
            (true, true, "✗", PRINT_STYLE.error, PrintType::Error)
          }
        };

      let hook_noise_level =
        hook.noise_level.as_ref().unwrap_or(global_noise_level);
      print(
        format!("\t{} {}", prefix.style(style), hook_name.style(style)),
        cli_noise_level.unwrap_or(hook_noise_level),
        print_type,
      );
      if !output.is_empty() && print_output {
        print(
          output,
          cli_noise_level.unwrap_or(hook_noise_level),
          PrintType::Info,
        );
      }

      if stop {
        exit(1);
      }
    }
  }

  Ok(())
}
