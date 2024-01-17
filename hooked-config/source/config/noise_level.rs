//! The noise level Hooked should output logs with.

use serde::{Deserialize, Serialize};

/// The noise level Hooked should output logs with.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
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
