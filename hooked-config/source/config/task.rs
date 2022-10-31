//! Task to perform in hooks.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Task to perform in hooks.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
  /// Command to execute directly.
  pub command: Option<String>,

  /// Path to a script to execute.
  pub script: Option<PathBuf>,
}
