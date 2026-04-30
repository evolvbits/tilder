<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

<p align="center">
  <img src=".github/brand/logo-text/compact/tilder-variation-3.svg" alt="tilder" width="180"/>
</p>

<h2 align="center">Declarative CLI for managing your Linux HOME directory.</h2>

[![Version](https://img.shields.io/badge/version-0.0.0-blue.svg)](https://github.com/evolvbits/tilder/releases)
![Rust](https://img.shields.io/badge/Rust-built-orange)

---

## Introduction

**Manage, reproduce, and control everything in your `$HOME` — declaratively.**

`Tilder` is a fast and minimalist CLI that lets you define the desired state of your HOME directory and automatically enforce it.

Instead of manually copying dotfiles, syncing folders, or rebuilding your environment from memory, you describe how your `$HOME` should look — and `Tilder` makes your system converge to that state.

> More powerful than *stow*. Simpler than *chezmoi*.

---

## Why Tilder?

Traditional dotfile tools focus on files. `Tilder` focuses on **state**.

It treats your HOME directory as a reproducible environment — not just a collection of configs.

With `Tilder`, you can:

* Define the structure and contents of your `$HOME`
* Keep files and directories consistently in sync
* Recreate your environment reliably at any time
* Eliminate configuration drift
* Manage more than dotfiles — manage your **entire home state**

---

## Philosophy

Your `$HOME` should be:

* **Declarative** — defined by intent, not manual steps
* **Reproducible** — rebuildable at any time
* **Consistent** — always matching your desired state
* **Simple** — without unnecessary complexity

`Tilder` turns your HOME directory into a predictable and controlled environment.

## About this repository

This public repository exists to:

* Provide verified and reproducible source code and binary versions
* Serve as the official download location
* Publish the `Tilder` logo for use on the official EvolvBits website
* Receive feedback, bug reports, and suggestions from users

All binaries published here are automatically compiled through a controlled CI pipeline to ensure authenticity and integrity.

For complete documentation and usage guides, please visit the official pages below.

## Official page

[https://evolvbits.github.io/products/tilder/](https://evolvbits.github.io/products/tilder/)

## Documentation

[https://evolvbits.github.io/products/tilder/documentation/](https://evolvbits.github.io/products/tilder/documentation/)

## Verifying Releases

All binaries are signed and can be verified.
See [SECURITY.md](SECURITY.md) for full verification instructions.

## Community

* [Contributing](CONTRIBUTING.md)
* [Development](DEVELOPMENT.md)
* [License Third-Party](LICENSE-THIRD-PARTY.md)
* [License](LICENSE)

---

© [EvolvBits](https://evolvbits.github.io) - All rights reserved.

<!-- "Tilder is source-available under the Elastic License 2.0. You may use, modify and contribute freely, but you may not sell or redistribute Tilder as a product or service." -->
