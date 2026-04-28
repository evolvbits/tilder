//! Application configuration and build-time constants

/// Application name
pub const APP_NAME: &str = "tilder";

/// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application description
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");