#!/usr/bin/env sh
# Tilder Installer
# Copyright (c) 2026 EvolvBits. All rights reserved.
# Author: William C. Canin <https://williamcanin.github.io>
#
# This file is part of Tilder and is licensed under the Elastic License 2.0
# (ELv2). You may not use this file except in compliance with the License.
#
# A copy of the license is available at:
# https://www.elastic.co/licensing/elastic-license
#
# --- Usage:
# Help
# curl -fsSL https://evolvbits.github.io/tilder/installer/linux.sh | sh -s -- --help
#
# List versions
# curl -fsSL https://evolvbits.github.io/tilder/installer/linux.sh | sh -s -- --versions
#
# Install a specific version
# curl -fsSL https://evolvbits.github.io/tilder/installer/linux.sh | sh -s -- 0.1.1
#
# Install the latest (no argument)
# curl -fsSL https://evolvbits.github.io/tilder/installer/linux.sh | sh
#
#
set -e

NAME="tilder"
REPO="evolvbits/tilder"
API_LATEST="https://api.github.com/repos/${REPO}/releases/latest"
API_RELEASES="https://api.github.com/repos/${REPO}/releases?per_page=50"
BINARY_NAME="tilder"
ARCH="x86_64"
INSTALLATION_DIR="$HOME/.local/bin"

# ----- libs -----
title() {
    printf "\033[0;35m[ %s\033[0m\n" "$1 ]"
}

info() {
    printf "\033[0;36m-> %s\033[0m$2" "$1"
}

finish() {
    printf "\033[0;32m* %s\033[0m\n" "$1"
}

warning() {
    printf "\033[0;33m! %s\033[0m$2" "$1"
}

error() {
    printf "\033[0;31mx %s\033[0m\n" "$1"
}

# ----- Help -----
show_help() {
    printf "\033[0;35mUsage:\033[0m\n"
    printf "  bash <(curl -fsSL https://evolvbits.github.io/smog/installer/linux.sh) [OPTION]\n\n"
    printf "\033[0;35mOptions:\033[0m\n"
    printf "  \033[0;36m<version>\033[0m        Install a specific version  (e.g. 0.1.1)\n"
    printf "  \033[0;36m--versions\033[0m       List all available versions\n"
    printf "  \033[0;36m--uninstall\033[0m      Uninstall %s\n" "$NAME"
    printf "  \033[0;36m--help\033[0m           Show this help message\n\n"
    printf "\033[0;35mExamples:\033[0m\n"
    printf "  # Install latest version\n"
    printf "  bash <(curl -fsSL https://evolvbits.github.io/smog/installer/linux.sh)\n\n"
    printf "  # Install a specific version\n"
    printf "  bash <(curl -fsSL https://evolvbits.github.io/smog/installer/linux.sh) 0.1.1\n\n"
    printf "  # List available versions\n"
    printf "  bash <(curl -fsSL https://evolvbits.github.io/smog/installer/linux.sh) --versions\n\n"
    printf "  # Uninstall\n"
    printf "  bash <(curl -fsSL https://evolvbits.github.io/smog/installer/linux.sh) --uninstall\n"
}

# ----- Fetch helper -----
fetch_url() {
    if command -v curl >/dev/null 2>&1; then
        curl -s "$1"
    else
        wget -qO- "$1"
    fi
}

# ----- Ignore root user -----
if [ "$(id -u)" -eq 0 ]; then
    error "Error: This script should not be run as root or with sudo."
    exit 1
fi

# ----- OS check -----
OS_TYPE=$(uname -s)
if [ "$OS_TYPE" != "Linux" ]; then
    error "Error: This script is only supported on Linux."
    info "Detected OS: " "${OS_TYPE}\n"
    exit 1
fi

# ----- ARCH check -----
CURRENT_ARCH=$(uname -m)
if [ "$CURRENT_ARCH" != "x86_64" ]; then
    error "Error: This binary is only for x86_64 architecture."
    exit 1
fi

# ----- Required check -----
for bin in curl wget; do
    if ! command -v "$bin" >/dev/null 2>&1; then
        error "Error: '$bin' not found."
        exit 1
    fi
done

# ----- Argument parsing -----
ARG="${1:-}"

case "$ARG" in
    --help|-h)
        show_help
        exit 0
        ;;

    --versions)
        title "$NAME Available Versions"
        VERSIONS=$(fetch_url "$API_RELEASES" | grep '"tag_name":' | sed -E 's/.*"v?([^"]+)".*/\1/')
        if [ -z "$VERSIONS" ]; then
            error "Error: Could not retrieve the list of versions from GitHub."
            exit 1
        fi
        printf "\033[0;36m%-12s\033[0m\n" "Version"
        printf '%s\n' "------------"
        printf '%s\n' "$VERSIONS" | while IFS= read -r v; do
            printf "  %s\n" "$v"
        done
        exit 0
        ;;

    --uninstall)
        title "$NAME Uninstall"

        detect_and_remove() {
            if [ -f "$1/$BINARY_NAME" ]; then
                info "Removing from: " "${1}\n"
                rm -fv "$1/$BINARY_NAME"
                return 0
            fi
            return 1
        }

        FOUND=0
        if detect_and_remove "$INSTALLATION_DIR"; then
            FOUND=1
        fi

        if [ "$FOUND" -eq 0 ]; then
            warning "No installation found." "\n"
        else
            finish "Uninstallation completed!"
        fi
        exit 0
        ;;

    "")
        # No argument — install latest
        VERSION_TAG=$(fetch_url "$API_LATEST" | grep '"tag_name":' | sed -E 's/.*"v?([^"]+)".*/\1/')
        if [ -z "$VERSION_TAG" ]; then
            error "Error: Could not retrieve the latest release version from GitHub."
            exit 1
        fi
        ;;

    *)
        # Strip leading 'v' if present, then validate semver format
        VERSION_TAG="${ARG#v}"
        case "$VERSION_TAG" in
            *[!0-9.]* | "")
                error "Error: Unknown option or invalid version: '$ARG'"
                info "Run with --help for usage." "\n"
                exit 1
                ;;
        esac
        ;;
esac

# ----- Download mode -----
title "$NAME Installation"
info "Version: " "${VERSION_TAG}\n"
info "Target file: " "${BINARY_NAME}-${VERSION_TAG}-${ARCH}\n"

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/v${VERSION_TAG}/${BINARY_NAME}-${VERSION_TAG}-${ARCH}"

info "Download link: " "${DOWNLOAD_URL}\n"
rm -f "$BINARY_NAME"

if command -v curl >/dev/null 2>&1; then
    if curl -L --fail --progress-bar "$DOWNLOAD_URL" -o "$BINARY_NAME"; then
        finish "Download completed successfully."
    else
        error "Error: Failed to download version '${VERSION_TAG}'. Check available versions with --versions."
        rm -f "$BINARY_NAME"
        exit 1
    fi
else
    if wget -q --show-progress "$DOWNLOAD_URL" -O "$BINARY_NAME"; then
        finish "Download completed successfully."
    else
        error "Error: Failed to download version '${VERSION_TAG}'. Check available versions with --versions."
        rm -f "$BINARY_NAME"
        exit 1
    fi
fi

# ----- Show SHA256SUM binary -----
if command -v sha256sum >/dev/null 2>&1; then
    info "SHA256SUM Binary: " ""; sha256sum "$BINARY_NAME"
fi

# ----- Install mode -----
mkdir -p "$INSTALLATION_DIR"
rm -f "$INSTALLATION_DIR/$BINARY_NAME"
cp -f "$BINARY_NAME" "${INSTALLATION_DIR}/"
chmod +x "$INSTALLATION_DIR/$BINARY_NAME"
rm -f "$BINARY_NAME"

# ----- Info mode -----
finish "Installation completed successfully!"
warning "$NAME was installed on: " ""; printf "%s\n" "$INSTALLATION_DIR"
warning "NOTE: " ""; printf "Add the path \"%s\" to your system's PATH.\n" "$INSTALLATION_DIR"
