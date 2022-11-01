//! General configuration definitions.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// General Hooked configuration.
#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct General {
  /// Path to the Hooked configuration file.
  pub config: PathBuf,

  /// The directory to use for hooks.
  pub directory: PathBuf,
}

impl Default for General {
  fn default() -> Self {
    Self {
      config: PathBuf::from("Hooked.toml"),
      directory: PathBuf::from("hooks"),
    }
  }
}
