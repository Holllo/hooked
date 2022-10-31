use {assert_cmd::Command, insta::assert_snapshot, test_case::test_case};

#[test_case("pre-commit", "pre-commit.toml", true; "pre-commit")]
fn test_hooked_run(hook_type: &str, config: &str, expect_failure: bool) {
  let config = format!("tests/samples/{}", config);

  let mut command = Command::cargo_bin("hooked").unwrap();
  command
    .args(["run", hook_type, "--config", &config])
    .env("NO_COLOR", "1");

  let assert = if expect_failure {
    command.assert().failure().code(1)
  } else {
    command.assert().success()
  };

  let stderr = std::str::from_utf8(&assert.get_output().stderr).unwrap();
  let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
  assert_snapshot!(hook_type, format!("{}{}", stdout, stderr));
}
