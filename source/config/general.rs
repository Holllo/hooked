//! General configuration definitions.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// General Hooked configuration.
#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct General {
  /// The directory to use for hooks.
  pub directory: PathBuf,
}

impl Default for General {
  fn default() -> Self {
    Self {
      directory: PathBuf::from("hooks"),
    }
  }
}
