#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPTS="$ROOT/scripts"
TEMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TEMP_DIR"' EXIT

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

manifest="$TEMP_DIR/Cargo.toml"
printf '[package]\nversion = "0.1.0"\n' > "$manifest"

if HEAD_REF=feature/no-release \
  CARGO_MANIFEST="$manifest" \
  GITHUB_OUTPUT="$TEMP_DIR/output" \
  bash "$SCRIPTS/validate_release.sh" >/dev/null 2>&1; then
  fail 'invalid release branch was accepted'
fi

mkdir -p "$TEMP_DIR/bin"
cat > "$TEMP_DIR/bin/git" <<'EOF'
#!/usr/bin/env bash
if [[ "${1:-}" == ls-remote ]]; then
  exit 2
fi
exec /usr/bin/git "$@"
EOF
chmod +x "$TEMP_DIR/bin/git"

PATH="$TEMP_DIR/bin:$PATH" \
  HEAD_REF=release/v0.1.0 \
  CARGO_MANIFEST="$manifest" \
  GITHUB_OUTPUT="$TEMP_DIR/output" \
  bash "$SCRIPTS/validate_release.sh" >/dev/null
grep -qx 'version=v0.1.0' "$TEMP_DIR/output" || fail 'release version was not emitted'

cat > "$TEMP_DIR/bin/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
url=''
output=''
while (($#)); do
  case "$1" in
    --url) url="$2"; shift 2 ;;
    --output) output="$2"; shift 2 ;;
    *) shift ;;
  esac
done
printf '%s\n' '{"login":"smoke","permissions":{"push":true}}' > "$output"
printf '200'
EOF
cat > "$TEMP_DIR/bin/jq" <<'EOF'
#!/usr/bin/env bash
if [[ "${2:-}" == *permissions* ]]; then
  printf 'true\n'
fi
EOF
cat > "$TEMP_DIR/bin/cargo" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >> "${RELEASE_TEST_LOG:?RELEASE_TEST_LOG is required}"
EOF
chmod +x "$TEMP_DIR/bin/curl" "$TEMP_DIR/bin/jq" "$TEMP_DIR/bin/cargo"

PATH="$TEMP_DIR/bin:$PATH" \
  RELEASE_TEST_LOG="$TEMP_DIR/cargo.log" \
  CARGO_REGISTRY_TOKEN=crate-token \
  GH_RELEASE_TOKEN=github-token \
  FORGEJO_TOKEN=codeberg-token \
  FORGEJO_API_URL=https://codeberg.example/api/v1 \
  FORGEJO_REPOSITORY=owner/repository \
  GITHUB_API_URL=https://api.github.com \
  GITHUB_REPOSITORY=owner/repository \
  VERSION=v0.1.0 \
  bash "$SCRIPTS/preflight_release.sh" >/dev/null

# Verify all four workspace members publish in dependency order
grep -qx 'package --locked -p senkai-core' "$TEMP_DIR/cargo.log" || fail 'core package did not run'
grep -qx 'publish --dry-run --locked -p senkai-core' "$TEMP_DIR/cargo.log" || fail 'core publish dry-run did not run'
grep -qx 'package --locked -p senkai-infrastructure' "$TEMP_DIR/cargo.log" || fail 'infrastructure package did not run'
grep -qx 'publish --dry-run --locked -p senkai-infrastructure' "$TEMP_DIR/cargo.log" || fail 'infrastructure publish dry-run did not run'
grep -qx 'package --locked -p senkai-presentation' "$TEMP_DIR/cargo.log" || fail 'presentation package did not run'
grep -qx 'publish --dry-run --locked -p senkai-presentation' "$TEMP_DIR/cargo.log" || fail 'presentation publish dry-run did not run'
grep -qx 'package --locked -p senkai' "$TEMP_DIR/cargo.log" || fail 'root package did not run'
grep -qx 'publish --dry-run --locked -p senkai' "$TEMP_DIR/cargo.log" || fail 'root publish dry-run did not run'

# Verify ordering: core before root
core_line=$(grep -nFx 'publish --dry-run --locked -p senkai-core' "$TEMP_DIR/cargo.log" | head -n1 | cut -d: -f1)
root_line=$(grep -nFx 'publish --dry-run --locked -p senkai' "$TEMP_DIR/cargo.log" | head -n1 | cut -d: -f1)
[ "$core_line" -lt "$root_line" ] || fail 'core must publish before root'

# Verify the real publish script publishes all four members in order
PATH="$TEMP_DIR/bin:$PATH" \
RELEASE_TEST_LOG="$TEMP_DIR/publish.log" \
  CARGO_REGISTRY_TOKEN=crate-token \
  VERSION=v0.1.0 \
  bash "$SCRIPTS/publish_crate.sh" >/dev/null



grep -qx 'publish --locked -p senkai-core' "$TEMP_DIR/publish.log" || fail 'core publish did not run'
grep -qx 'publish --locked -p senkai-infrastructure' "$TEMP_DIR/publish.log" || fail 'infrastructure publish did not run'
grep -qx 'publish --locked -p senkai-presentation' "$TEMP_DIR/publish.log" || fail 'presentation publish did not run'
grep -qx 'publish --locked -p senkai' "$TEMP_DIR/publish.log" || fail 'root publish did not run'

core_line=$(grep -nFx 'publish --locked -p senkai-core' "$TEMP_DIR/publish.log" | head -n1 | cut -d: -f1)
root_line=$(grep -nFx 'publish --locked -p senkai' "$TEMP_DIR/publish.log" | head -n1 | cut -d: -f1)
[ "$core_line" -lt "$root_line" ] || fail 'core must publish before root'

printf 'release script tests: PASS\n'

