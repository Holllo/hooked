//! General configuration definitions.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::NoiseLevel;

/// General Hooked configuration.
#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct General {
  /// Path to the Hooked configuration file.
  pub config: PathBuf,

  /// The directory to use for hooks.
  pub directory: PathBuf,

  /// The noise level tasks should output logs with by default.
  pub noise_level: NoiseLevel,

  /// Path to a script template for use with the install subcommand.
  pub template: Option<PathBuf>,
}

impl Default for General {
  fn default() -> Self {
    Self {
      config: PathBuf::from("Hooked.toml"),
      directory: PathBuf::from("hooks"),
      noise_level: NoiseLevel::default(),
      template: None,
    }
  }
}
