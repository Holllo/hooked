use std::path::PathBuf;

use {color_eyre::Result, hooked::Config};

use {insta::assert_debug_snapshot, test_case::test_case};

#[test_case("defaults" ; "defaults")]
fn test_config_parsing(file_name: &str) -> Result<()> {
  let file = PathBuf::from(format!("tests/parsing/{file_name}.toml"));
  let config = Config::from_toml_file(file)?;
  assert_debug_snapshot!(format!("{file_name}"), config);
  Ok(())
}
