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

mkdir -p "$TEMP_DIR/bin"
MOCK_LOG="$TEMP_DIR/mock_log"

cat > "$TEMP_DIR/bin/git" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == "log" ]]; then
  cat "$MOCK_LOG"
  exit 0
fi
exec /usr/bin/git "\$@"
EOF
chmod +x "$TEMP_DIR/bin/git"

test_commits() {
  local commits="$1"
  printf '%s\n' "$commits" > "$MOCK_LOG"
  PATH="$TEMP_DIR/bin:$PATH" bash "$SCRIPTS/check_commit_messages.sh"
}

# 1. Valid conventional commits
test_commits "feat: add awesome feature" >/dev/null || fail "conventional commit failed"
test_commits "fix(ui): handle empty list" >/dev/null || fail "conventional commit with scope failed"
test_commits "chore: bump dependencies" >/dev/null || fail "chore commit failed"

# 2. Merge commits (both capital and lowercase)
test_commits "Merge pull request #56 from chore/split-command-types into main" >/dev/null || fail "Merge commit failed"
test_commits "merge PR #57: shell_quoter refactor and port/cli cleanup" >/dev/null || fail "merge lowercase commit failed"

# 3. Invalid commit messages must be rejected
if test_commits "invalid non-conventional message" >/dev/null 2>&1; then
  fail "invalid commit message was erroneously accepted"
fi

if test_commits "wip: work in progress" >/dev/null 2>&1; then
  fail "wip commit message was erroneously accepted"
fi

printf 'commit message check tests: PASS\n'
