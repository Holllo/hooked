//! The `run` subcommand.

use std::{io::Read, process::exit};

use {
  color_eyre::{eyre::eyre, Result},
  hooked_config::{Config, ExitAction},
  owo_colors::{OwoColorize, Style},
  subprocess::{Exec, Redirection},
  supports_color::Stream,
};

use crate::utilities::{globset_from_strings, plural};

/// The `run` subcommand.
pub fn hooked_run(config: Config, hook_type: String) -> Result<()> {
  let (success_style, warn_style, error_style, skipped_style) =
    if let Some(_support) = supports_color::on(Stream::Stdout) {
      let shared_style = Style::new().bold();
      (
        shared_style.green(),
        shared_style.yellow(),
        shared_style.red(),
        shared_style.blue(),
      )
    } else {
      (Style::new(), Style::new(), Style::new(), Style::new())
    };

  if hook_type == "pre-commit" {
    let hook_count = config.pre_commit.len();
    println!(
      "Hooked: Running {} pre-commit {}.",
      hook_count,
      plural(hook_count, "hook", None)
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
          println!(
            "\t{} {}",
            "≫".style(skipped_style),
            hook_name.style(skipped_style)
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

      let (stop, print_output, prefix, style) =
        match (exit_status.success(), hook.on_failure) {
          (true, _) => (false, false, "✓", success_style),
          (false, ExitAction::Continue) => (false, true, "⚠", warn_style),
          (false, ExitAction::Stop) => (true, true, "✗", error_style),
        };

      println!("\t{} {}", prefix.style(style), hook_name.style(style));
      if !output.is_empty() && print_output {
        println!("{}", output);
      }

      if stop {
        exit(1);
      }
    }
  }

  Ok(())
}
