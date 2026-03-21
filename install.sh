#!/bin/sh
set -e

REPO="louisescher/effectengine"
BINARY="effectengine"

# Fetch latest effectengine tag
VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases" \
  | grep '"tag_name"' \
  | grep 'effectengine-v' \
  | head -1 \
  | cut -d'"' -f4 \
  | sed 's/effectengine-v//')

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
  arm64|aarch64) ARCH="aarch64" ;;
  x86_64)        ARCH="x86_64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

case "$OS" in
  darwin) OS="macos" ;;
  linux)  OS="linux" ;;
  *) echo "Unsupported OS: $OS. On Windows, use scoop."; exit 1 ;;
esac

TAG="effectengine-v${VERSION}"
URL="https://github.com/${REPO}/releases/download/${TAG}/${BINARY}-${OS}-${ARCH}.tar.gz"

echo "Installing effectengine v${VERSION} (${OS}/${ARCH})..."
curl -sL "$URL" | tar xz
chmod +x "$BINARY"

INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
mv "$BINARY" "${INSTALL_DIR}/${BINARY}"
echo "Installed to ${INSTALL_DIR}/${BINARY}"
