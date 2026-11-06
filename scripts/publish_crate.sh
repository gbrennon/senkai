#!/usr/bin/env bash
set -euo pipefail

readonly TOKEN="${CARGO_REGISTRY_TOKEN:-}"
readonly VERSION="${VERSION:?VERSION is required}"
readonly EXPECTED_VERSION="${VERSION#v}"

if [[ -z "$TOKEN" ]]; then
  printf 'CARGO_REGISTRY_TOKEN is not configured\n' >&2
  exit 1
fi

manifest_version="$(sed -nE 's/^version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -n 1)"
if [[ "$manifest_version" != "$EXPECTED_VERSION" ]]; then
  printf 'Cargo.toml version %s does not match release %s\n' \
    "$manifest_version" "$VERSION" >&2
  exit 1
fi

# Publish workspace members in dependency order: core first, then
# infrastructure and presentation (which depend on core), then the root
# crate (which depends on all three). crates.io rejects path dependencies,
# so each member must be published explicitly.
cargo publish --locked -p senkai-core
cargo publish --locked -p senkai-infrastructure
cargo publish --locked -p senkai-presentation
cargo publish --locked -p senkai
