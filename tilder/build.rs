// (c) 2026 EvolvBits. All rights reserved.

use std::{env::var, fs, path::Path, process::Command};

fn git_commit() -> String {
  let hash = Command::new("git")
    .args(["rev-parse", "HEAD"])
    .output()
    .ok()
    .and_then(|output| {
      if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
      } else {
        None
      }
    })
    .unwrap_or_else(|| "unknown".to_string());

  // I only take the first 8 characters of the commit hash.
  hash[..8].to_string()
}

fn git_date() -> String {
  Command::new("git")
    .args(["show", "-s", "--format=%cs", "HEAD"])
    .output()
    .ok()
    .and_then(|o| {
      o.status
        .success()
        .then(|| String::from_utf8_lossy(&o.stdout).trim().to_string())
    })
    .unwrap_or_else(|| "unknown".into())
}

fn metadata(key: &str) -> Result<String, Box<dyn std::error::Error>> {
  // CARGO_MANIFEST_DIR points to where the build.rs file is located (the root of the workspace).
  let manifest_dir = var("CARGO_MANIFEST_DIR")?;

  // Go up one level to get the Cargo.toml file from the workspace.
  let workspace_cargo_toml = Path::new(&manifest_dir)
    .parent()
    .unwrap()
    .join("Cargo.toml");

  let content = fs::read_to_string(&workspace_cargo_toml)?;
  let toml: toml::Value = content.parse()?;
  let value = toml
    .get("workspace")
    .and_then(|w| w.get("metadata"))
    .and_then(|w| w.get("tilder"))
    .and_then(|m| m.get(key))
    .and_then(|c| c.as_str())
    .unwrap_or("Unknown");

  Ok(value.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!(
    "cargo:rustc-env=APP_COPYRIGHT={}",
    metadata("copyright").unwrap_or_else(|_| "unknown".into())
  );

  println!(
    "cargo:rustc-env=APP_MAINTAINER={}",
    metadata("maintainer").unwrap_or_else(|_| "unknown".into())
  );

  println!(
    "cargo:rustc-env=APP_DOCUMENTATION={}",
    metadata("documentation").unwrap_or_else(|_| "unknown".into())
  );

  println!(
    "cargo:rustc-env=APP_KEYWORDS={}",
    metadata("keywords").unwrap_or_else(|_| "unknown".into())
  );

  println!(
    "cargo:rustc-env=APP_CATEGORIES={}",
    metadata("categories").unwrap_or_else(|_| "unknown".into())
  );

  println!("cargo:rustc-env=APP_COMMIT={}", git_commit());

  println!("cargo:rustc-env=APP_LAST_UPDATE={}", git_date());

  let profile = var("PROFILE").unwrap_or_else(|_| "unknown".into());
  println!("cargo:rustc-env=BUILD_PROFILE={}", profile);

  Ok(())
}
