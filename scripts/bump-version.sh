#!/bin/bash
set -e

NEW_VERSION=$1

# Update version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" Cargo.toml

# Regenerate Cargo.lock
cargo generate-lockfile
