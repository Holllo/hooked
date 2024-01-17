//! Action to take on hook exit.

use serde::{Deserialize, Serialize};

/// Action to take on hook exit.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitAction {
  /// Regardless of the hook's exit code, allow Hooked to continue.
  Continue,
  /// Stop on a non-zero hook exit code.
  Stop,
}

impl Default for ExitAction {
  fn default() -> Self {
    Self::Stop
  }
}
