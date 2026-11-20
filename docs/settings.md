# Settings

Senkai stores settings as string key/value pairs in a TOML file. The default
location is `$XDG_CONFIG_HOME/senkai/settings.toml` when `XDG_CONFIG_HOME` is
set; otherwise it is `~/.config/senkai/settings.toml`.

## List settings

```bash
sk settings list
```

Names are printed in alphabetical order.

## Read a setting

```bash
sk settings get theme
```

The command prints the value or reports an error when the setting does not
exist.

## Set a setting

```bash
sk settings set theme dark
```

The command loads the existing TOML file, replaces or adds the named value, and
writes the complete settings file back to disk. Values are stored as TOML
strings; malformed TOML and non-string values are rejected.

## Display settings in the TUI

```bash
sk tui settings
```

The TUI loads the persisted settings and displays sorted `name = value` rows.
This view is read-only; use `sk settings set <name> <value>` to change a value.
