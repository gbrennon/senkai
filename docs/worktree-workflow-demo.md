# Worktree workflow demo

This walkthrough explains the recorded Senkai workflow for managing Git worktrees
inside tmux.

![Senkai worktree workflow](gifs/senkai-worktree-workflow.gif)

The recording uses tmux's default theme, Bash as the default shell, and a
temporary Git repository. It shows creating two worktrees, returning to the main
window, resuming a worktree, and removing both worktrees.

The temporary tmux configuration sets `default-shell` and `default-command` to
`/bin/bash`. It also sets the Senkai plugin option `@worktree-command` to
`/bin/bash`, so newly created worktree windows use Bash.

## Create the first worktree

Senkai creates a Git worktree for `feat/one` and opens a dedicated tmux window
named `wt-feat/one`.

```text
sk create-worktree feat/one
```

The new shell starts in the worktree directory, so commands run there operate on
the `feat/one` branch.

## Create another worktree

The second command creates `feat/two` and opens its own tmux window.

```text
sk create-worktree feat/two
```

Each branch has an independent directory and tmux window while sharing the same
Git repository history.

## Return to the main window

After each worktree operation, the recording returns to the main tmux window
with `prefix + 0`. The worktree windows remain available for later selection.

## Resume the worktree

Running the create command for an existing worktree resumes it instead of
creating a duplicate:

```text
sk create-worktree feat/one
```

Senkai finds the registered worktree, selects its tmux window, and returns to the
existing branch.

## Remove worktrees

Run cleanup to open the multi-select removal picker:

```text
sk cleanup
```

Select each worktree with Space, press Enter, and confirm the removal. Senkai
removes the Git worktree directories, deletes their local branches, and closes
the corresponding tmux windows.

The recording uses the plugin keybindings; the commands above are equivalent
CLI operations. For the interactive keybinding workflow, see [Create a worktree](create-worktree.md)
and [Clean up worktrees](cleanup-worktrees.md).
