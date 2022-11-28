//! Miscellaneous utilities.

use color_eyre::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};

/// Simple function to create a pluralized string.
pub fn plural(count: usize, singular: &str, plural: Option<&str>) -> String {
  if count == 1 {
    return singular.to_string();
  }

  if let Some(plural) = plural {
    return plural.to_string();
  }

  format!("{singular}s")
}

/// Create a [`GlobSet`] from a list of strings.
pub fn globset_from_strings(input: &[String]) -> Result<GlobSet> {
  let mut builder = GlobSetBuilder::new();
  for glob in input {
    builder.add(Glob::new(&glob)?);
  }

  builder.build().map_err(Into::into)
}
