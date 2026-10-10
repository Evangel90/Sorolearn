#!/usr/bin/env bash
# Prepares a container so `sorolearn` starts instantly and works offline.
set -euo pipefail
cd "$(dirname "$0")/.."

rustup update stable --no-self-update
rustup default stable
# For `sorolearn graduate` (deploys to Stellar testnet).
rustup target add wasm32v1-none

STELLAR_CLI_VERSION=28.1.0
if ! command -v stellar >/dev/null; then
  # The prebuilt Linux binary links against libdbus (used for OS keychains).
  if ! ldconfig -p | grep -q libdbus-1.so.3; then
    SUDO=$([ "$(id -u)" -eq 0 ] && echo "" || echo sudo)
    $SUDO apt-get update -qq
    $SUDO apt-get install -y -qq libdbus-1-3
  fi
  arch=$(uname -m)
  curl -fsSL "https://github.com/stellar/stellar-cli/releases/download/v${STELLAR_CLI_VERSION}/stellar-cli-${STELLAR_CLI_VERSION}-${arch}-unknown-linux-gnu.tar.gz" \
    | tar -xz -C "${CARGO_HOME:-$HOME/.cargo}/bin" stellar
fi
stellar --version | head -1

cargo install --path . --locked --force
sorolearn setup
