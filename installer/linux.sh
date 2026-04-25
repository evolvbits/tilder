
#!/usr/bin/env bash

# Author: William C. Canin <https://williamcanin.github.io>

set -euo pipefail

NAME="tilder"
REPO="evolvbits/tilder"
API_URL="https://api.github.com/repos/${REPO}/releases/latest"
BINARY_NAME="tilder"
ARCH="x86_64"
INSTALLATION_DIR="$HOME/.local/bin"
REQUIRED=("curl" "wget")

# ----- libs -----
title () {
	printf "\e[0;35m[ %s\e[0m\n" "$1 ]"
}

info () {
	printf "\e[0;36m-> %s\e[0m$2" "$1"
}

finish () {
	printf "\e[0;32m* %s\e[0m\n" "$1"
}

warning () {
	printf "\e[0;33m! %s\e[0m$2" "$1"
}

error () {
	printf "\e[0;31mx %s\e[0m\n" "$1"
}

# ----- Ignore root user -----
if [ "$EUID" -eq 0 ]; then
  error "Error: This script should not be run as root or with sudo."
  exit 1
fi

# ----- Required check -----
for bin in "${REQUIRED[@]}"; do
  if ! command -v "$bin" >/dev/null 2>&1; then
    error "Error: '$bin' not found."
    exit 1
  fi
done

# ----- Uninstall mode -----
if [ "${1:-}" == "--uninstall" ]; then
    title "$NAME Uninstall"

    detect_and_remove () {
        local DIR="$1"
        if [ -f "$DIR/$BINARY_NAME" ]; then
            info "Removing from: " "${DIR}\n"
            rm -fv "$DIR/$BINARY_NAME"
            return 0
        fi
        return 1
    }

    FOUND=0

    for d in \
        $INSTALLATION_DIR
    do
        detect_and_remove "$d" && FOUND=1
    done

    if [ "$FOUND" -eq 0 ]; then
        warning "No installation found." "\n"
    else
        finish "Uninstallation completed!"
    fi

    exit 0
fi

# ----- Download mode -----
title "$NAME Installation"
if command -v curl >/dev/null 2>&1; then
		VERSION_TAG=$(curl -s "$API_URL" | grep '"tag_name":' | sed -E 's/.*"v?([^"]+)".*/\1/')
else
		VERSION_TAG=$(wget -qO- "$API_URL" | grep '"tag_name":' | sed -E 's/.*"v?([^"]+)".*/\1/')
fi

if [ -z "$VERSION_TAG" ]; then
		error "Error: Could not retrieve the latest release version from GitHub."
		exit 1
fi
info "Latest version: " "${VERSION_TAG}\n"
info "Target file: " "${BINARY_NAME}-${VERSION_TAG}-${ARCH}\n"

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/v${VERSION_TAG}/${BINARY_NAME}-${VERSION_TAG}-${ARCH}"

rm -f $BINARY_NAME
info "Download link: " "$DOWNLOAD_URL\n"
if command -v curl >/dev/null 2>&1; then
	if curl -L --fail --progress-bar "$DOWNLOAD_URL" -o "$BINARY_NAME"; then
			finish "Download completed successfully."
	else
			error "Error: Failed to download the latest release."
			rm -f "$BINARY_NAME"
			exit 1
	fi
else
	if wget -q --show-progress "$DOWNLOAD_URL" -O "$BINARY_NAME"; then
			finish "Download completed successfully."
	else
			error "Error: Failed to download the latest release."
			rm -f "$BINARY_NAME"
			exit 1
	fi
fi
info "Target file rename to: " "${BINARY_NAME}\n"

# ----- Show SHA256SUM Binary -----
if command -v sha256sum >/dev/null 2>&1; then
	info "SHA256SUM Binary: " ""; sha256sum $BINARY_NAME
fi

# ----- Install mode -----
mkdir -p "$INSTALLATION_DIR"
rm -f "$INSTALLATION_DIR/$BINARY_NAME"
cp -f $BINARY_NAME "${INSTALLATION_DIR}/"
chmod +x "$INSTALLATION_DIR/$BINARY_NAME"
rm -f $BINARY_NAME

# ----- Info mode -----
finish "Installation completed successfully!"
warning "$NAME was installed on: " ""; printf "%s\n" "$INSTALLATION_DIR"
warning "NOTE: " "";  printf "Add the path \"%s\" to your system's PATH.\n" "$INSTALLATION_DIR"
