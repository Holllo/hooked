//! Action to take on hook exit.

use serde::{Deserialize, Serialize};

/// Action to take on hook exit.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ExitAction {
  Continue,
  Stop,
}

impl Default for ExitAction {
  fn default() -> Self {
    Self::Stop
  }
}
