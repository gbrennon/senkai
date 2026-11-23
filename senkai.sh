        choose)
            path="$(command sk choose --print-path "$@")" || return
        create|create-worktree)
            path="$(command sk create-worktree --print-path "$@")" || return