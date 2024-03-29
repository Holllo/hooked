//! Configuration structs and logic.

use std::{fs::read_to_string, path::Path};

use {
  color_eyre::Result,
  serde::{Deserialize, Serialize},
};

mod exit_action;
mod general;
mod noise_level;
mod pre_commit;
mod task;

pub use exit_action::*;
pub use general::*;
pub use noise_level::*;
pub use pre_commit::*;
pub use task::*;

/// The main Hooked configuration struct.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
  /// General Hooked configuration.
  pub general: General,

  /// Pre-commit hooks.
  pub pre_commit: Vec<PreCommit>,
}

impl Config {
  /// Read a file and parse it with [`toml`].
  pub fn from_toml_file<P>(file: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    toml::from_str(&read_to_string(file)?).map_err(Into::into)
  }
}
