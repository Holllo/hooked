//! The noise level Hooked should output logs with.

use serde::{Deserialize, Serialize};

/// The noise level Hooked should output logs with.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NoiseLevel {
  /// Output only errors.
  Quiet,
  /// Output everything.
  Loud,
  /// Print a list of tasks and output warnings and errors, this is the default.
  Standard,
  /// The same as [`NoiseLevel::Standard`] except don't output task names or
  /// warnings.
  Minimal,
}

impl Default for NoiseLevel {
  fn default() -> Self {
    Self::Standard
  }
}

// Implement `From<String>` so we can use Clap's automatic parsing in the CLI.
impl From<String> for NoiseLevel {
  fn from(value: String) -> Self {
    match value.to_lowercase().as_str() {
      "quiet" => Self::Quiet,
      "loud" => Self::Loud,
      "standard" => Self::Standard,
      "minimal" => Self::Minimal,
      _ => NoiseLevel::default(),
    }
  }
}
