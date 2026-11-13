# senkai

![senkai TUI screenshot](docs/gifs/senkai-demo.gif)

senkai is a terminal user interface (TUI) for creating, switching between,
and cleaning up [git worktrees][git-worktree]. It also includes a tmux plugin
that opens the TUI from tmux keybindings and manages the tmux windows for each
worktree. The current multiplexer integration targets tmux exclusively; support
for additional multiplexers is planned.

## About the name

*senkai* draws inspiration from *Bleach*'s **Senkaimon** (穿界門), the
World-Piercing Gate that connects worlds. That influence fits a tool that moves
between worktree environments. The spelling also echoes *senkai* (旋回), a
Japanese word for circling, rotation, revolution, or a banking turn—an apt
metaphor for rotating your tmux workspace between Git worktrees.

## Features

- Create worktrees from a branch name and open them in dedicated tmux windows.
- Resume and switch to Git-registered worktrees, including worktrees outside the
  configured directory. The picker resolves paths from the active repository
  context and avoids listing that resolved root itself.
- Keep managing external worktrees: switch to them, inspect their branches,
  and remove them through the same cleanup flow.
- Customize the worktree directory, keybindings, and shell used by new windows.
- Clean up merged or stale worktrees without leaving tmux windows behind.

**Demo**: `prefix + W` opens a picker — type a branch name, press Enter, and
you're in a fresh tmux window pointing at that worktree. `prefix + D` opens a
multi-select cleanup picker: press Space to toggle worktrees, Enter to review,
then choose Yes or No.

---

## Install and run

`sk tui` is the standalone TUI command. Run it from a Git repository to open the
worktree picker:

```bash
cd ~/my-project
sk tui
```

Choose one of these installation methods.

### Cargo

Install the published crate with Cargo:

```bash
cargo install senkai
sk tui
```

### tmux plugin

Add the plugin to `~/.tmux.conf`, then install or reload it with your TPM
workflow (`prefix + I` for TPM):

```tmux
set -g @plugin 'gbrennon/senkai'
```

The plugin finds an installed `sk` binary, downloads the latest compatible
release artifact, or builds `sk` with Cargo when needed. Once installed, use
`prefix + W` to open the worktree TUI or `prefix + D` to open the cleanup TUI.

The plugin needs tmux >= 3.2, git >= 2.28, bash >= 4.0, `curl`, and `tar`.

### Download an artifact

The latest release artifact is available for Linux x86_64:

```bash
curl -fL \
  https://github.com/gbrennon/senkai/releases/latest/download/sk-x86_64-unknown-linux-gnu.tar.gz \
  -o sk.tar.gz
tar -xzf sk.tar.gz
install -m 755 sk ~/.local/bin/sk
sk tui
```

Ensure `~/.local/bin` is in `PATH` before running `sk`.

Plain CLI commands do not start Ratatui. Use `sk choose <name>` to create or
resume a named worktree, or use `sk choose` for a numbered terminal prompt. Use
`sk tui`, `sk tui choose`, or `sk tui cleanup` when you want the Ratatui interface.

SSH authentication uses the agent available when the plugin is loaded. The
plugin persists `SSH_AUTH_SOCK` into the invoking tmux session environment so
Git fetches from worktree windows reuse your loaded SSH keys instead of
prompting for a password. If you start or restart your agent later, reload the
plugin from a shell with the agent exported (or source `senkai.tmux` again) to
refresh the session socket path.

---

## Keybindings

| Keys           | What it does                  | Details |
|----------------|-------------------------------|---------|
| `prefix` + `W` | Create or switch to worktree  | [docs](docs/create-worktree.md) |
| `prefix` + `D` | Remove merged/stale worktrees | [docs](docs/cleanup-worktrees.md) |

> `prefix` is `Ctrl-b` by default.  Override the keys by setting these
> variables in `~/.tmux.conf` **before** the plugin line — for example:
>
> ```tmux
> set -g @worktree-key 'T'           # prefix + T to pick a worktree
> set -g @worktree-cleanup-key 'X'   # prefix + X to clean up
> set -g @worktree-dir 'my-trees'    # custom worktree directory
> ```
>
> More details: [Customise the keys](docs/customise-keys.md).
> You can also [change the shell](docs/customise-shell.md) that opens in new windows,
> [change the worktree directory](docs/customise-worktree-dir.md), or
> [customize Senkai colors](docs/customise-theme.md).

---

## Quick start

```
cd ~/my-project
prefix + W
```
Type `feat/foo` -> Enter. A worktree is created (by default at
`.worktrees/feat-foo`) and a new tmux window `wt-feat/foo` opens there.
Press `prefix + W` again to switch between worktrees, including existing
worktrees outside `.worktrees`; press `prefix + D` to clean up.

> Examples use [conventional branch][conv-branch] prefixes (`feat/`, `fix/`,
> `chore/`), but any branch name works — `my-branch` or `bugfix/login` are
> equally valid.

---

## More

| Topic | |
|-------|---|
| [Creating worktrees](docs/create-worktree.md) | Step-by-step walkthrough |
| [Switching worktrees](docs/switch-worktrees.md) | Using the picker as a dashboard |
| [Cleaning up](docs/cleanup-worktrees.md) | Removing worktrees and branches |
| [Workflow demo](docs/worktree-workflow-demo.md) | Recorded create, resume, and removal flow |
| [Customise keys](docs/customise-keys.md) | Change `W` / `D` bindings |
| [Customise shell](docs/customise-shell.md) | Launch `bash`, `zsh`, `vim`, `emacs`, etc. |
| [Customise worktree dir](docs/customise-worktree-dir.md) | Use a different worktree directory |
| [Customize colors](docs/customise-theme.md) | Match Senkai to standard tmux styles |
| [How it works](docs/how-it-works.md) | Under-the-hood architecture |
| [Releasing](docs/releasing.md) | Release branches and publishing |

---

## Testing

Run the Rust test suite from the project root:

```bash
cargo test -- --test-threads=1
```

For the coverage-enforced project test recipe, install `just` and
`cargo-llvm-cov`, then run `just test`. Tests create and tear down temporary
git repositories in `/tmp` — no project files are touched.

---

## License

MIT

[git-worktree]: https://git-scm.com/docs/git-worktree
[tpm]: https://github.com/tmux-plugins/tpm
[conv-branch]: https://conventional-branch.github.io/
