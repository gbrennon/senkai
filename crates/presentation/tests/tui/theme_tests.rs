use ratatui::style::Color;
use senkai_core::error::Result;
use senkai_core::ports::multiplexer_port::MultiplexerPort;
use senkai_presentation::tui::theme::Theme;

struct FakeMultiplexer {
    status_left: Option<String>,
    status_style: Option<String>,
}

impl FakeMultiplexer {
    fn with_style(style: &str) -> Self {
        Self {
            status_left: None,
            status_style: Some(style.to_string()),
        }
    }

    fn with_options(status_left: &str, status_style: &str) -> Self {
        Self {
            status_left: Some(status_left.to_string()),
            status_style: Some(status_style.to_string()),
        }
    }

    fn unavailable() -> Self {
        Self {
            status_left: None,
            status_style: None,
        }
    }
}

impl MultiplexerPort for FakeMultiplexer {
    fn show_error(&self, _message: &str) -> Result<()> {
        unimplemented!()
    }

    fn display_popup(
        &self,
        _width: &str,
        _height: &str,
        _directory: &str,
        _command: &str,
    ) -> Result<()> {
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

    fn show_environment(&self, _variable: &str) -> Result<Option<String>> {
        unimplemented!()
    }

    fn current_pane_path(&self) -> Result<String> {
        unimplemented!()
    }

    fn run(&self, arguments: &[&str]) -> Result<(i32, String, String)> {
        let style = match arguments.last().copied() {
            Some("status-left") => self.status_left.as_ref(),
            Some("status-style") => self.status_style.as_ref(),
            _ => None,
        };
        style
            .map(|style| (0, style.clone(), String::new()))
            .ok_or_else(|| senkai_core::error::Error::new("tmux unavailable"))
    }

    fn kill_window(&self, _name: &str) -> Result<()> {
        unimplemented!()
    }
}

mod tests {
    use super::*;

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

    #[test]
    fn multiplexer_theme_uses_inline_status_left_style() {
        let theme = Theme::from_multiplexer(&FakeMultiplexer::with_options(
            "#[fg=colour234,bg=colour166,bold] user",
            "bg=green,fg=black",
        ));

        assert_eq!(theme.text, Color::Indexed(234));
        assert_eq!(theme.highlight_fg, Color::Indexed(166));
        assert_eq!(theme.highlight_bg, Color::Indexed(234));
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
}
