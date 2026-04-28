//! Application configuration and build-time constants
//! These values are populated by build.rs
//!

/// Application name
pub const APP_NAME: &str = "tilder";

/// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application description
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// /// Project repository
// pub const APP_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

// /// Project site
// pub const PROJECT_SITE: &str = env!("PROJECT_SITE");

// /// Copyright year
// pub const APP_COPYRIGHT: &str = env!("APP_COPYRIGHT");

// /// Project maintainer
// pub const PROJECT_MAINTAINER: &str = env!("PROJECT_MAINTAINER");

// /// Git commit hash
// pub const GIT_COMMIT: &str = env!("GIT_COMMIT");

// /// Git commit date
// pub const GIT_DATE: &str = env!("GIT_DATE");

// /// Build profile (debug/release)
// pub const BUILD_PROFILE: &str = env!("BUILD_PROFILE");
