//! Pre-commit hook definitions.

use serde::{Deserialize, Serialize};

use crate::{ExitAction, Task};

/// A pre-commit hook.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreCommit {
  /// List of globs to check against staged files.
  #[serde(default)]
  pub git_staged: Vec<String>,

  /// Display name for this hook.
  pub name: Option<String>,

  /// What to do when the hook exits with a non-zero status code.
  #[serde(default)]
  pub on_failure: ExitAction,

  /// Task to perform when this hook is called.
  #[serde(flatten)]
  pub task: Task,
}
