# Customize Senkai colors

Senkai reads standard tmux style options only when it is running inside a tmux
client. Outside tmux, the standalone CLI uses its own default terminal theme.

## Configure standard tmux styles

Set these options in `~/.tmux.conf` before loading Senkai:

```tmux
set -g status-style 'bg=green,fg=black'
set -g pane-active-border-style 'fg=green'
set -g popup-style 'bg=green,fg=black'
set -g popup-border-style 'fg=green'
```

Senkai uses the options in this order:

1. `@sk-popup-style` or `@sk-popup-border-style`.
2. `popup-style` or `popup-border-style`.
3. `status-style` for popup text and background.
4. `pane-active-border-style` for the popup border.
5. tmux terminal-default colors when no configured style is available.

The border style controls borders only. Senkai keeps selected-row foreground and
background colors from the popup or status style so a dark border cannot make
selection text unreadable.

Reload the running server after changing the configuration:

```sh
tmux source-file ~/.tmux.conf
```

## Add a Senkai-only override

Use the `@sk-` prefix only when Senkai needs a deliberate exception to the
shared tmux theme:

```tmux
set -g @sk-popup-style 'bg=colour236,fg=colour250'
set -g @sk-popup-border-style 'fg=colour45'
```

Senkai accepts tmux color names, `colour0` through `colour255`, hexadecimal RGB
values, and `default`. The complete style syntax is documented in the
[tmux Getting Started guide][tmux-getting-started].

[tmux-getting-started]: https://github.com/tmux/tmux/wiki/Getting-Started
