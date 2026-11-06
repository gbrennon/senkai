#!/usr/bin/env bash
set -euo pipefail

readonly UPLOAD_URL="${RELEASE_UPLOAD_URL:?RELEASE_UPLOAD_URL is required}"
readonly TOKEN="${RELEASE_TOKEN:?RELEASE_TOKEN is required}"
readonly ASSET="${RELEASE_ASSET:?RELEASE_ASSET is required}"
readonly AUTH_SCHEME="${RELEASE_AUTH_SCHEME:-Bearer}"

if [[ ! -f "$ASSET" ]]; then
  printf 'Release asset not found: %s\n' "$ASSET" >&2
  exit 1
fi

name="$(basename "$ASSET")"
encoded_name="$(jq -nr --arg name "$name" '$name | @uri')"
curl --fail --silent --show-error --request POST \
  --url "${UPLOAD_URL}?name=${encoded_name}" \
  --header "Authorization: ${AUTH_SCHEME} ${TOKEN}" \
  --header 'Content-Type: application/gzip' \
  --data-binary "@${ASSET}"
printf 'Uploaded release asset: %s\n' "$name"
