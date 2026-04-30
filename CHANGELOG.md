# Changelog

All notable changes to this project will be documented in this file.

The format is based on Conventional Commits.

## [Unreleased] - 2026-04-29

### Bug Fixes

- installer: fix(installer): hide temp file path from sha256sum output ([d1e7c98](https://github.com/evolvbits/tilder/commit/d1e7c9812595dfe5c4162a49bdbc10bf07f3f368))
  - Extract only the hash via awk '{print $1}'
  - Display it through the info() helper instead of raw sha256sum output
  - Code formatting (indentation)

- fix: correcting value variables ([a291d17](https://github.com/evolvbits/tilder/commit/a291d17879565f1d0176be41e6184a03a88293dd))

### Features

- feat: new design and variations brand Tilder ([eb8115e](https://github.com/evolvbits/tilder/commit/eb8115e84fba175b34517f8cea5cb8104b5ec945))

- installer: feat(installer): install to /usr/local/bin with sudo support ([a90e17e](https://github.com/evolvbits/tilder/commit/a90e17eb827237694cb45c8c629c046be4025dc9))
  - Change INSTALLATION_DIR from ~/.local/bin to /usr/local/bin
  - Add privilege helper: skip sudo if already root, otherwise require it
  - Prefix install/uninstall commands with $SUDO
  - Replace temp binary in $PWD with mktemp + trap for automatic cleanup

- installer: feat(installer): add version selection, --versions and --help flags, migrate to POSIX sh ([7b18629](https://github.com/evolvbits/tilder/commit/7b18629c053485e6f4a8b73b4a8ef360ede5dd0f))
  - Replace bash with POSIX sh (#!/usr/bin/env sh, set -e)
  - Add `<version>` argument to install any specific release
  - Add --versions flag to list all available GitHub releases
  - Add --help / -h flag with usage examples
  - Default behaviour (no args) still installs latest
  - Replace $EUID with $(id -u) for POSIX compliance
  - Replace bash arrays with plain for-in loop

- feat: add Rust workspace structure and core functionality ([10d7dac](https://github.com/evolvbits/tilder/commit/10d7dac8f34e0aa9560b2ab6d7091c990b666660))
  - Add workspace Cargo.toml with Rust 2024 edition and dependencies
  - Create tilder-core crate with cross-platform userprofile! macro
  - Add main tilder crate with CLI entry point and build.rs
  - Implement workspace metadata and build configuration
  - Add proper dependency management with Cargo.lock

### Refactoring

- refactor: rename tilder_core to tilder-core following Rust naming conventions ([b2b1d7b](https://github.com/evolvbits/tilder/commit/b2b1d7beb1549c5cd2eec9d48a746528fcde9e0e))

- refactor: rename tilder-core to tilder_core following Rust convention ([927d3b8](https://github.com/evolvbits/tilder/commit/927d3b83eb3af30b2365f7f85aa36b4131986007))
