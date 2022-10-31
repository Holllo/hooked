//! The `run` subcommand.

use std::{io::Read, process::exit};

use {
  color_eyre::{eyre::eyre, Result},
  hooked_library::{Config, ExitAction},
  owo_colors::{OwoColorize, Style},
  subprocess::{Exec, Redirection},
  supports_color::Stream,
};

/// The `run` subcommand.
pub fn hooked_run(config: Config, hook_type: String) -> Result<()> {
  let (success_style, warn_style, error_style) =
    if let Some(_support) = supports_color::on(Stream::Stdout) {
      let shared_style = Style::new().bold();
      (
        shared_style.green(),
        shared_style.yellow(),
        shared_style.red(),
      )
    } else {
      (Style::new(), Style::new(), Style::new())
    };

  if hook_type == "pre-commit" {
    println!(
      "Hooked: Running {} pre-commit hooks.",
      config.pre_commit.len()
    );

    for hook in config.pre_commit {
      let hook_name = hook.name.unwrap_or_else(|| "Unnamed Hook".to_string());

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
