//! Shared logic for printing output to the terminal.

use {
  hooked_config::NoiseLevel, lazy_static::lazy_static, owo_colors::Style,
  std::fmt::Display, supports_color::Stream,
};

/// The available types to print output as.
#[derive(Debug, Eq, PartialEq)]
pub enum PrintType {
  /// Print the output as an error line.
  Error,
  /// Print the output as a warning line.
  Warn,
  /// Print the output as an information line.
  Info,
}

/// The available print styles for colorized output.
#[derive(Debug)]
pub struct PrintStyles {
  /// The style for errored hooks output.
  pub error: Style,
  /// The style for skipped hooks output.
  pub skipped: Style,
  /// The style for succesful hooks output.
  pub success: Style,
  /// The style for hooks with warnings.
  pub warn: Style,
}

lazy_static! {
  pub static ref PRINT_STYLE: PrintStyles = {
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

    PrintStyles {
      error: error_style,
      skipped: skipped_style,
      success: success_style,
      warn: warn_style,
    }
  };
}

/// Print something to the terminal according to a given [`NoiseLevel`] and
/// [`PrintType`].
pub fn print<D: Display>(
  something: D,
  noise_level: &NoiseLevel,
  print_type: PrintType,
) {
  let should_print = match (noise_level, print_type) {
    // Only output errors under the quiet noise level.
    (NoiseLevel::Quiet, PrintType::Error) => true,
    (NoiseLevel::Quiet, _) => false,
    // Output everything under loud.
    (NoiseLevel::Loud | NoiseLevel::Standard | NoiseLevel::Minimal, _) => true,
  };

  if !should_print {
    return;
  }

  println!("{something}")
}
