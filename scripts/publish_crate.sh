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

# Publish a single crate, retrying until the crates.io index has
# propagated the previously published dependency. First-time publication
# of interdependent crates fails if the dependent's index entry is not
# yet queryable.
publish_crate() {
  local crate="$1"
  local attempt=1
  local max_attempts="${PUBLISH_MAX_ATTEMPTS:-5}"
  local delay="${PUBLISH_RETRY_DELAY:-20}"
  while [[ $attempt -le $max_attempts ]]; do
    if cargo publish --locked -p "$crate"; then
      return 0
    fi
    if [[ $attempt -lt $max_attempts ]]; then
      printf 'Publish %s attempt %d/%d failed, retrying in %ds...\n' \
        "$crate" "$attempt" "$max_attempts" "$delay" >&2
      sleep "$delay"
    fi
    attempt=$((attempt + 1))
  done
  printf 'Publish %s failed after %d attempts\n' "$crate" "$max_attempts" >&2
  return 1
}

# Publish workspace members in dependency order: core first, then
# infrastructure and presentation (which depend on core), then the root
# crate (which depends on all three). crates.io rejects path dependencies,
# so each member must be published explicitly.
publish_crate senkai-core
publish_crate senkai-infrastructure
publish_crate senkai-presentation
publish_crate senkai
