#!/usr/bin/env bash
# Prepares a container so `sorolearn` starts instantly and works offline.
set -euo pipefail
cd "$(dirname "$0")/.."

rustup update stable --no-self-update
rustup default stable
cargo install --path . --locked --force
sorolearn setup
