#!/usr/bin/env bash
set -euo pipefail

require_value() {
  local name="$1"
  local value="$2"
  if [[ -z "$value" ]]; then
    printf '%s is not configured\n' "$name" >&2
    exit 1
  fi
}

probe_identity() {
  local label="$1"
  local url="$2"
  local token="$3"
  local scheme="$4"
  local response_file
  local status

  response_file="$(mktemp)"
  trap 'rm -f "$response_file"' RETURN
  status="$(curl --silent --show-error --request GET \
    --url "${url%/}/user" \
    --header "Authorization: ${scheme} ${token}" \
    --output "$response_file" --write-out '%{http_code}')"
  if [[ "$status" != 2* ]]; then
    printf '%s token authentication failed (HTTP %s)\n' "$label" "$status" >&2
    exit 1
  fi
  printf '%s token authentication: PASS\n' "$label"
}

probe_repository_write_access() {
  local label="$1"
  local url="$2"
  local repository="$3"
  local token="$4"
  local scheme="$5"
  local response_file
  local status
  local push_access

  response_file="$(mktemp)"
  trap 'rm -f "$response_file"' RETURN
  status="$(curl --silent --show-error --request GET \
    --url "${url%/}/repos/${repository}" \
    --header "Authorization: ${scheme} ${token}" \
    --header 'Accept: application/json' \
    --output "$response_file" --write-out '%{http_code}')"
  if [[ "$status" != 2* ]]; then
    printf '%s repository access failed (HTTP %s)\n' "$label" "$status" >&2
    exit 1
  fi
  push_access="$(jq -r '.permissions.push // false' "$response_file")"
  if [[ "$push_access" != true ]]; then
    printf '%s token lacks repository write permission\n' "$label" >&2
    exit 1
  fi
  printf '%s repository write access: PASS\n' "$label"
}

require_value CARGO_REGISTRY_TOKEN "${CARGO_REGISTRY_TOKEN:-}"
require_value GH_RELEASE_TOKEN "${GH_RELEASE_TOKEN:-}"
require_value FORGEJO_TOKEN "${FORGEJO_TOKEN:-}"
require_value FORGEJO_API_URL "${FORGEJO_API_URL:-}"
require_value FORGEJO_REPOSITORY "${FORGEJO_REPOSITORY:-}"
require_value GITHUB_REPOSITORY "${GITHUB_REPOSITORY:-}"

probe_identity Codeberg "$FORGEJO_API_URL" "$FORGEJO_TOKEN" token
probe_repository_write_access Codeberg "$FORGEJO_API_URL" "$FORGEJO_REPOSITORY" \
  "$FORGEJO_TOKEN" token
probe_identity GitHub "${GITHUB_API_URL:-https://api.github.com}" \
  "$GH_RELEASE_TOKEN" Bearer
probe_repository_write_access GitHub "${GITHUB_API_URL:-https://api.github.com}" \
  "$GITHUB_REPOSITORY" "$GH_RELEASE_TOKEN" Bearer

# Full package+dry-run on leaf (core). Dependents cannot cargo-package
# pre-publication because crates.io has no senkai-core index entry yet.
cargo package --locked -p senkai-core
cargo publish --dry-run --locked -p senkai-core

# Validate dependent manifests have required publish metadata without
# invoking cargo package (which would resolve against crates.io index).
for member in senkai-infrastructure senkai-presentation senkai; do
  manifest="crates/${member#senkai-}/Cargo.toml"
  [[ -f "$manifest" ]] || manifest="Cargo.toml"
  grep -qE "^license[[:space:]]*=" "$manifest" || { printf "%s missing license\n" "$member" >&2; exit 1; }
  grep -qE "^repository[[:space:]]*=" "$manifest" || { printf "%s missing repository\n" "$member" >&2; exit 1; }
  grep -qE "^readme[[:space:]]*=" "$manifest" || { printf "%s missing readme\n" "$member" >&2; exit 1; }
done
printf 'Cargo package and publish dry-run: PASS\n'
printf 'Note: crates.io token permission is confirmed by the real cargo publish step.\n'
