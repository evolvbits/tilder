//! Application configuration and build-time constants

/// Application name
pub const APP_NAME: &str = "tilder";

/// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application description
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Application maintainer
pub const APP_MAINTAINER: &str = env!("APP_MAINTAINER");

/// Application git commit
pub const APP_COMMIT: &str = env!("APP_COMMIT");

/// Application git last commit date
pub const APP_LAST_UPDATE: &str = env!("APP_LAST_UPDATE");

/// Application repository app
pub const APP_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// Application LICENSE
pub const APP_LICENSE: &str = env!("CARGO_PKG_LICENSE");

/// Application description
pub const APP_COPYRIGHT: &str = env!("APP_COPYRIGHT");

/// Application homepage
pub const APP_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

// /// Application documentation
// pub const APP_DOCUMENTATION: &str = env!("APP_DOCUMENTATION");
