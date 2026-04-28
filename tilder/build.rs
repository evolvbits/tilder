// (c) 2026 EvolvBits. All rights reserved.

use std::{env, fs, path::Path, process::Command};

fn git_commit() -> String {
  Command::new("git")
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
    .unwrap_or_else(|| "unknown".to_string())
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

fn git_cliff() {
  let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

  if profile == "release" {
    // Notify the role to rebuild if the root Cargo.toml changes.
    println!("cargo:rerun-if-changed=../Cargo.toml");

    let version = env!("CARGO_PKG_VERSION");

    // We execute the command by explicitly pointing to the files in the root directory. (../)
    let status = Command::new("git-cliff")
      .args([
        "--offline",
        "--config",
        "cliff.toml", // Path to the config in the root directory.
        "--output",
        "CHANGELOG.md",
      ])
      .current_dir("..") // FORCE the command to run in the workspace root.
      .status();

    match status {
      Ok(s) if s.success() => println!("cargo:warning=CHANGELOG.md updated to v{}", version),
      _ => {
        println!("cargo:warning=Failed to update CHANGELOG.md. Check if git-cliff is working.")
      }
    }
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // CARGO_MANIFEST_DIR points to where the build.rs file is located (the root of the workspace).
  let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

  // Go up one level to get the Cargo.toml file from the workspace.
  let workspace_cargo_toml = Path::new(&manifest_dir)
    .parent()
    .unwrap()
    .join("Cargo.toml");

  let content = fs::read_to_string(&workspace_cargo_toml)?;
  let toml: toml::Value = content.parse()?;

  let copyright = toml
    .get("workspace")
    .and_then(|w| w.get("metadata"))
    .and_then(|m| m.get("copyright"))
    .and_then(|c| c.as_str())
    .unwrap_or("Unknown");

  println!("cargo:rustc-env=APP_COPYRIGHT={}", copyright);

  let maintainer = toml
    .get("workspace")
    .and_then(|w| w.get("metadata"))
    .and_then(|m| m.get("maintainer"))
    .and_then(|c| c.as_str())
    .unwrap_or("Unknown");

  println!("cargo:rustc-env=PROJECT_MAINTAINER={}", maintainer);

  let site = toml
    .get("workspace")
    .and_then(|w| w.get("metadata"))
    .and_then(|m| m.get("site"))
    .and_then(|c| c.as_str())
    .unwrap_or("Unknown");

  println!("cargo:rustc-env=PROJECT_SITE={}", site);

  let commit = git_commit();
  println!("cargo:rustc-env=GIT_COMMIT={}", commit);

  let commit_date = git_date();
  println!("cargo:rustc-env=GIT_DATE={}", commit_date);

  let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".into());
  println!("cargo:rustc-env=BUILD_PROFILE={}", profile);

  git_cliff();

  Ok(())
}
