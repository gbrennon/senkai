#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TEMP_DIR"' EXIT

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

make_fixture() {
  local fixture="$1"
  mkdir -p "$fixture/bin" "$fixture/src"
  cp "$ROOT/worktree.tmux" "$fixture/worktree.tmux"
  printf '[package]\nname = "senkai"\nversion = "0.1.0"\n' > "$fixture/Cargo.toml"
  printf 'fn main() {}\n' > "$fixture/src/main.rs"

  cat > "$fixture/bin/tmux" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >> "${TMUX_LOG:?TMUX_LOG is required}"
case "${1:-}" in
  display-message)
    printf 'test-session\n'
    exit 0
    ;;
  show-option|list-keys) exit 1 ;;
  *) exit 0 ;;
esac
EOF
  cat > "$fixture/bin/cargo" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >> "${CARGO_LOG:?CARGO_LOG is required}"
mkdir -p target/release
printf '#!/usr/bin/env bash\nexit 0\n' > target/release/sk
chmod +x target/release/sk
EOF
  cat > "$fixture/bin/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${CURL_FAIL_DOWNLOAD:-false}" == true && "$*" == *'releases/download'* ]]; then
  exit 1
fi
if [[ "$*" != *'releases/download'* ]]; then
  printf '{\n  "tag_name": "v0.4.0",\n  "assets": []\n}\n'
  exit 0
fi
output=''
for ((i = 1; i <= $#; i++)); do
  eval "arg=\${$i}"
  if [[ "$arg" == -o ]]; then
    ((i++))
    eval "output=\${$i}"
  fi
done
mkdir -p "$(dirname "$output")"
tar -czf "$output" -C "${ARCHIVE_SOURCE:?ARCHIVE_SOURCE is required}" sk
EOF
  chmod +x "$fixture/bin"/*
}

run_installer() {
  local fixture="$1"
  local test_path="${TEST_PATH:-$fixture/bin:/usr/bin:/bin}"
  (cd "$fixture" && HOME="$fixture/home" PATH="$test_path" \
    CARGO_LOG="$fixture/cargo.log" ARCHIVE_SOURCE="$fixture/archive" \
    TMUX_LOG="$fixture/tmux.log" SSH_AUTH_SOCK="$fixture/agent.sock" \
    bash "$fixture/worktree.tmux")
}

fixture="$TEMP_DIR/installed"
make_fixture "$fixture"
mkdir -p "$fixture/archive" "$fixture/home"
printf '#!/usr/bin/env bash\nexit 0\n' > "$fixture/bin/sk"
chmod +x "$fixture/bin/sk"
run_installer "$fixture"
[[ ! -s "$fixture/cargo.log" ]] || fail 'Cargo build ran despite installed CLI'
[[ ! -e "$fixture/home/.tmux/plugins/senkai/sk" ]] \
  || fail 'Plugin copied a second binary despite installed CLI'
grep -Fq -- "bind-key -T prefix W run-shell -b $fixture/bin/sk choose" "$fixture/tmux.log" \
  || fail 'Choose binding did not use installed CLI'
grep -Fq -- "bind-key -T prefix D run-shell -b $fixture/bin/sk cleanup" "$fixture/tmux.log" \
  || fail 'Cleanup binding did not use installed CLI'

fixture="$TEMP_DIR/default-path"
make_fixture "$fixture"
mkdir -p "$fixture/archive" "$fixture/home/.cargo/bin"
printf '#!/usr/bin/env bash\nexit 0\n' > "$fixture/home/.cargo/bin/sk"
chmod +x "$fixture/home/.cargo/bin/sk"
run_installer "$fixture"
[[ ! -s "$fixture/cargo.log" ]] || fail 'Cargo build ran despite default-path CLI'
[[ ! -e "$fixture/home/.tmux/plugins/senkai/sk" ]] \
  || fail 'Plugin copied a second binary despite default-path CLI'
grep -Fq -- "bind-key -T prefix W run-shell -b $fixture/home/.cargo/bin/sk choose" "$fixture/tmux.log" \
  || fail 'Choose binding did not use default-path CLI'
grep -Fq -- "bind-key -T prefix D run-shell -b $fixture/home/.cargo/bin/sk cleanup" "$fixture/tmux.log" \
  || fail 'Cleanup binding did not use default-path CLI'

fixture="$TEMP_DIR/plugin-path"
make_fixture "$fixture"
mkdir -p "$fixture/archive" "$fixture/home/.cargo/bin" \
  "$fixture/home/.tmux/plugins/senkai"
printf '#!/usr/bin/env bash\nexit 0\n' > "$fixture/home/.cargo/bin/sk"
printf '#!/usr/bin/env bash\nexit 0\n' > "$fixture/home/.tmux/plugins/senkai/sk"
chmod +x "$fixture/home/.cargo/bin/sk" \
  "$fixture/home/.tmux/plugins/senkai/sk"
TEST_PATH="$fixture/home/.tmux/plugins/senkai:$fixture/bin:/usr/bin:/bin" \
  run_installer "$fixture"
[[ ! -s "$fixture/cargo.log" ]] || fail 'Cargo build ran despite PATH CLI'
grep -Fq -- "bind-key -T prefix W run-shell -b $fixture/home/.tmux/plugins/senkai/sk choose" "$fixture/tmux.log" \
  || fail 'Choose binding did not prefer PATH CLI over Cargo default path'
grep -Fq -- "bind-key -T prefix D run-shell -b $fixture/home/.tmux/plugins/senkai/sk cleanup" "$fixture/tmux.log" \
  || fail 'Cleanup binding did not prefer PATH CLI over Cargo default path'

fixture="$TEMP_DIR/download"
make_fixture "$fixture"
mkdir -p "$fixture/archive" "$fixture/home"
printf '#!/usr/bin/env bash\nexit 0\n' > "$fixture/archive/sk"
chmod +x "$fixture/archive/sk"
run_installer "$fixture"
[[ -x "$fixture/home/.tmux/plugins/senkai/sk" ]] || fail 'GitHub asset was not installed'
[[ ! -s "$fixture/cargo.log" ]] || fail 'Cargo build ran despite successful asset download'
grep -Fq -- "set-environment -t test-session SSH_AUTH_SOCK $fixture/agent.sock" "$fixture/tmux.log" \
  || fail 'SSH agent socket was not persisted to the tmux session'

fixture="$TEMP_DIR/fallback"
make_fixture "$fixture"
mkdir -p "$fixture/archive" "$fixture/home"
printf '#!/usr/bin/env bash\nexit 0\n' > "$fixture/archive/sk"
chmod +x "$fixture/archive/sk"
CURL_FAIL_DOWNLOAD=true run_installer "$fixture"
[[ -x "$fixture/home/.tmux/plugins/senkai/sk" ]] || fail 'source fallback did not install binary'
grep -q 'build --release' "$fixture/cargo.log" || fail 'source fallback did not invoke cargo build'
grep -Fq -- "set-environment -t test-session SSH_AUTH_SOCK $fixture/agent.sock" "$fixture/tmux.log" \
  || fail 'SSH agent socket was not persisted during source fallback'

printf 'installer script smoke tests: PASS\n'
