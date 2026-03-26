#!/usr/bin/env bash
set -euo pipefail

# Load .env if present
if [[ -f .env ]]; then
  set -o allexport
  source .env
  set +o allexport
fi

# Require CARGO_REGISTRY_TOKEN
if [[ -z "${CARGO_REGISTRY_TOKEN:-}" ]]; then
  echo "error: CARGO_REGISTRY_TOKEN is not set" >&2
  exit 1
fi

# Ensure working tree is clean
if [[ -n "$(git status --porcelain)" ]]; then
  echo "error: working tree is dirty, commit or stash changes first" >&2
  exit 1
fi

# Run checks
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# Dry run first
cargo publish --dry-run --token "$CARGO_REGISTRY_TOKEN"

# Publish
cargo publish --token "$CARGO_REGISTRY_TOKEN"
