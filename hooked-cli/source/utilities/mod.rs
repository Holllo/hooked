//! Miscellaneous utilities.

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
