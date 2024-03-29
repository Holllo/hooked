use {
  hooked_config::{Config, ExitAction, NoiseLevel, PreCommit, Task},
  toml::to_string_pretty,
};

use insta::assert_snapshot;

#[test]
fn test_serialize() {
  let pre_commit_command = PreCommit {
    name: Some("Command Test".to_string()),
    noise_level: None,
    on_failure: ExitAction::Continue,
    staged: vec!["*.txt".to_string()],
    task: Task {
      command: Some("exit 0".to_string()),
      script: None,
    },
  };

  let pre_commit_script = PreCommit {
    name: Some("Script Test".to_string()),
    noise_level: Some(NoiseLevel::Loud),
    on_failure: ExitAction::Stop,
    staged: vec![],
    task: Task {
      command: None,
      script: Some("test.sh".into()),
    },
  };

  let config = Config {
    general: Default::default(),
    pre_commit: vec![pre_commit_command, pre_commit_script],
  };

  assert_snapshot!("serialize", to_string_pretty(&config).unwrap());
}
