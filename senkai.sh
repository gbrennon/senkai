#!/usr/bin/env bash

# Wrap the executable so directory changes happen in the calling shell.
sk() {
    if [ -n "${TMUX:-}" ]; then
        command sk "$@"
        return
    fi

    case "${1:-}" in
        choose)
            shift
            local path
            path="$(command sk choose --print-path "$@")" || return
            [ -n "$path" ] || return 1
            cd -- "$path"
            ;;
        create|create-worktree)
            shift
            local path
            path="$(command sk create-worktree --print-path "$@")" || return
            [ -n "$path" ] || return 1
            cd -- "$path"
            ;;
        *)
            command sk "$@"
            ;;
    esac
}
