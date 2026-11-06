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

# Report whether $crate is already published at $EXPECTED_VERSION on crates.io.
# Exits nonzero on network errors so set -e aborts rather than skipping blindly.
already_published() {
  local crate="$1"
  local code
  code="$(RELEASE_VERSION="$EXPECTED_VERSION" python3 - "$crate" <<'PY'
import os, sys, urllib.error, urllib.request
crate = sys.argv[1]
version = os.environ["RELEASE_VERSION"]
try:
    urllib.request.urlopen(f"https://crates.io/api/v1/crates/{crate}/{version}", timeout=10)
    print(200)
except urllib.error.HTTPError as e:
    print(e.code)
except Exception as exc:
    print(0, file=sys.stderr)
    sys.exit(1)
PY
)"
  [[ "$code" == "200" ]] && return 0
  [[ "$code" == "404" ]] && return 1
  printf 'crates.io check for %s returned %s, aborting\n' "$crate" "$code" >&2
  exit 1
}

publish_crate() {
  local crate="$1"
  if already_published "$crate"; then
    printf 'Skipping %s: already published at %s\n' "$crate" "$EXPECTED_VERSION"
    return 0
  fi
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
