use ratatui::style::{Color, Modifier};
use senkai_core::ports::multiplexer_port::MultiplexerPort;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Theme {
    pub(crate) text: Color,
    pub(crate) border: Color,
    pub(crate) prompt: Color,
    pub(crate) header: Color,
    pub(crate) highlight_fg: Color,
    pub(crate) highlight_bg: Color,
    pub(crate) highlight_modifier: Modifier,
}

pub(crate) const TEXT: Color = Color::White;
pub(crate) const BORDER: Color = Color::Cyan;
pub(crate) const PROMPT: Color = Color::Cyan;
pub(crate) const HEADER: Color = Color::Yellow;
pub(crate) const HIGHLIGHT_FG: Color = Color::Black;
pub(crate) const HIGHLIGHT_BG: Color = Color::Cyan;
pub(crate) const HIGHLIGHT_MODIFIER: Modifier = Modifier::BOLD;

impl Default for Theme {
    fn default() -> Self {
        Self {
            text: TEXT,
            border: BORDER,
            prompt: PROMPT,
            header: HEADER,
            highlight_fg: HIGHLIGHT_FG,
            highlight_bg: HIGHLIGHT_BG,
            highlight_modifier: HIGHLIGHT_MODIFIER,
        }
    }
}

impl Theme {
    pub(crate) fn from_multiplexer(terminal: &dyn MultiplexerPort) -> Self {
        let style = terminal
            .run(&["show-options", "-v", "status-style"])
            .ok()
            .and_then(|(status, output, _)| (status == 0).then_some(output));
        Self::from_tmux_style(style.as_deref())
    }

    pub(crate) fn from_tmux_style(style: Option<&str>) -> Self {
        let Some(style) = style else {
            return Self::default();
        };
        let Some(foreground) = style_color(style, "fg") else {
            return Self::default();
        };
        let Some(background) = style_color(style, "bg") else {
            return Self::default();
        };

        Self {
            text: foreground,
            border: foreground,
            prompt: foreground,
            header: foreground,
            highlight_fg: background,
            highlight_bg: foreground,
            ..Self::default()
        }
    }
}

fn style_color(style: &str, key: &str) -> Option<Color> {
    style.split(',').find_map(|part| {
        let (part_key, value) = part.trim().split_once('=')?;
        (part_key == key)
            .then(|| parse_color(value.trim()))
            .flatten()
    })
}

fn parse_color(value: &str) -> Option<Color> {
    match value.to_ascii_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "brightblack" | "grey" | "gray" => Some(Color::DarkGray),
        "brightred" => Some(Color::LightRed),
        "brightgreen" => Some(Color::LightGreen),
        "brightyellow" => Some(Color::LightYellow),
        "brightblue" => Some(Color::LightBlue),
        "brightmagenta" => Some(Color::LightMagenta),
        "brightcyan" => Some(Color::LightCyan),
        "brightwhite" => Some(Color::White),
        value if value.starts_with("colour") => value
            .strip_prefix("colour")
            .and_then(|index| index.parse::<u8>().ok())
            .map(Color::Indexed),
        value if value.starts_with('#') => parse_rgb(value),
        _ => None,
    }
}

fn parse_rgb(value: &str) -> Option<Color> {
    (value.len() == 7).then(|| {
        let red = u8::from_str_radix(&value[1..3], 16).ok()?;
        let green = u8::from_str_radix(&value[3..5], 16).ok()?;
        let blue = u8::from_str_radix(&value[5..7], 16).ok()?;
        Some(Color::Rgb(red, green, blue))
    })?
}

#[cfg(test)]
mod tests {
    use super::*;
    use senkai_core::error::Result;
    use senkai_core::ports::multiplexer_port::MultiplexerPort;

    struct FakeMultiplexer {
        style: Option<String>,
    }

    impl FakeMultiplexer {
        fn with_style(style: &str) -> Self {
            Self {
                style: Some(style.to_string()),
            }
        }

        fn unavailable() -> Self {
            Self { style: None }
        }
    }

    impl MultiplexerPort for FakeMultiplexer {
        fn show_error(&self, _msg: &str) -> Result<()> {
            unimplemented!()
        }

        fn display_popup(&self, _width: &str, _height: &str, _dir: &str, _cmd: &str) -> Result<()> {
            unimplemented!()
        }

        fn resolve_worktree_dir(&self) -> String {
            unimplemented!()
        }

        fn resolve_shell_command(&self) -> String {
            unimplemented!()
        }

        fn select_or_create_window(&self, _name: &str, _path: &str, _command: &str) -> Result<()> {
            unimplemented!()
        }

        fn get_option(&self, _key: &str) -> Option<String> {
            unimplemented!()
        }

        fn show_environment(&self, _var: &str) -> Result<Option<String>> {
            unimplemented!()
        }

        fn current_pane_path(&self) -> Result<String> {
            unimplemented!()
        }

        fn run(&self, _args: &[&str]) -> Result<(i32, String, String)> {
            self.style
                .as_ref()
                .map(|style| (0, style.clone(), String::new()))
                .ok_or_else(|| senkai_core::error::Error::new("tmux unavailable"))
        }

        fn kill_window(&self, _name: &str) -> Result<()> {
            unimplemented!()
        }
    }

    #[test]
    fn multiplexer_theme_uses_active_status_style() {
        let theme = Theme::from_multiplexer(&FakeMultiplexer::with_style("fg=green,bg=black"));

        assert_eq!(theme.text, Color::Green);
        assert_eq!(theme.highlight_fg, Color::Black);
        assert_eq!(theme.highlight_bg, Color::Green);
    }

    #[test]
    fn multiplexer_theme_falls_back_when_tmux_is_unavailable() {
        let theme = Theme::from_multiplexer(&FakeMultiplexer::unavailable());

        assert_eq!(theme, Theme::default());
    }

    #[test]
    fn default_theme_preserves_current_palette() {
        assert_eq!(Theme::default().text, Color::White);
        assert_eq!(Theme::default().border, Color::Cyan);
        assert_eq!(Theme::default().header, Color::Yellow);
        assert_eq!(Theme::default().highlight_fg, Color::Black);
        assert_eq!(Theme::default().highlight_bg, Color::Cyan);
    }

    #[test]
    fn tmux_style_maps_named_foreground_and_background() {
        let theme = Theme::from_tmux_style(Some("fg=yellow,bg=blue"));

        assert_eq!(theme.text, Color::Yellow);
        assert_eq!(theme.border, Color::Yellow);
        assert_eq!(theme.prompt, Color::Yellow);
        assert_eq!(theme.header, Color::Yellow);
        assert_eq!(theme.highlight_fg, Color::Blue);
        assert_eq!(theme.highlight_bg, Color::Yellow);
    }

    #[test]
    fn tmux_style_maps_indexed_and_rgb_colors() {
        let indexed = Theme::from_tmux_style(Some("fg=colour123,bg=colour45"));
        assert_eq!(indexed.text, Color::Indexed(123));
        assert_eq!(indexed.highlight_fg, Color::Indexed(45));

        let rgb = Theme::from_tmux_style(Some("fg=#12abef,bg=#010203"));
        assert_eq!(rgb.text, Color::Rgb(0x12, 0xab, 0xef));
        assert_eq!(rgb.highlight_fg, Color::Rgb(1, 2, 3));
    }

    #[test]
    fn invalid_or_partial_tmux_style_preserves_default_colors() {
        let invalid = Theme::from_tmux_style(Some("fg=not-a-color,bg=also-invalid"));
        assert_eq!(invalid, Theme::default());

        let partial = Theme::from_tmux_style(Some("fg=yellow"));
        assert_eq!(partial, Theme::default());
    }
}
