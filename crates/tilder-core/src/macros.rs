// (c) 2026 EvolvBits. All rights reserved.
//! Cross-platform path construction macros for `tilder`.
//!
//! Provides the [`userprofile!`] macro that resolves the current user's home
//! directory on both Unix (`$HOME`) and Windows (`%USERPROFILE%`) and appends
//! one or more path components to it.

/// Builds a [`std::path::PathBuf`] rooted at the current user's home directory.
///
/// Accepts one or more string expressions that are joined with the
/// platform-native separator (`/` on Unix) and appended to the
/// home directory path.
///
/// # Panics
///
/// Panics if the `HOME` (Unix) environment variable
/// is not set.
///
/// # Examples
///
/// ```rust
/// use tilder_core::userprofile;
///
/// // Resolves to e.g. "/home/william/.config.json"
/// let path = userprofile!(".config.json");
///
/// // Resolves to e.g. "/home/william/app1/config/settings.json"
/// let nested = userprofile!("app1", "config", "settings.json");
/// ```
#[macro_export]
macro_rules! userprofile {
  ($($part:expr),*) => {
    {
      let base_path =   std::path::PathBuf::from(std::env::var("HOME").unwrap());
      let path_parts = vec![$($part),*];
      let separator = if cfg!(windows) { r"\" } else { "/" };
      let path_str = path_parts.join(separator);
      base_path.join(path_str)
    }
  };
}
