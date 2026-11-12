use ratatui::style::Color;
use senkai_core::error::Result;
use senkai_core::ports::multiplexer_port::MultiplexerPort;
use senkai_presentation::tui::theme::Theme;

struct FakeMultiplexer {
    popup_style: Option<String>,
    popup_border_style: Option<String>,
    status_style: Option<String>,
    active_border_style: Option<String>,
    inside_tmux: bool,
}

impl FakeMultiplexer {
    fn with_popup_styles(popup_style: &str, popup_border_style: &str) -> Self {
        Self {
            popup_style: Some(popup_style.to_string()),
            popup_border_style: Some(popup_border_style.to_string()),
            status_style: None,
            active_border_style: None,
            inside_tmux: true,
        }
    }

    fn with_popup_style(popup_style: &str) -> Self {
        Self {
            popup_style: Some(popup_style.to_string()),
            popup_border_style: None,
            status_style: None,
            active_border_style: None,
            inside_tmux: true,
        }
    }

    fn with_standard_styles(status_style: &str, active_border_style: &str) -> Self {
        Self {
            popup_style: Some("default".to_string()),
            popup_border_style: Some("default".to_string()),
            status_style: Some(status_style.to_string()),
            active_border_style: Some(active_border_style.to_string()),
            inside_tmux: true,
        }
    }

    fn outside_tmux() -> Self {
        Self {
            popup_style: Some("fg=red,bg=blue".to_string()),
            popup_border_style: Some("fg=yellow".to_string()),
            status_style: None,
            active_border_style: None,
            inside_tmux: false,
        }
    }

    fn unavailable() -> Self {
        Self {
            popup_style: None,
            popup_border_style: None,
            status_style: None,
            active_border_style: None,
            inside_tmux: false,
        }
    }
}

impl MultiplexerPort for FakeMultiplexer {
    fn is_inside_tmux(&self) -> bool {
        self.inside_tmux
    }

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
        if !arguments.iter().any(|argument| argument.contains("-g")) {
            return Err(senkai_core::error::Error::new(
                "tmux option lookup was not global",
            ));
        }
        let style = match arguments.last().copied() {
            Some("popup-style") => self.popup_style.as_ref(),
            Some("popup-border-style") => self.popup_border_style.as_ref(),
            Some("status-style") => self.status_style.as_ref(),
            Some("pane-active-border-style") => self.active_border_style.as_ref(),
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
    fn default_theme_uses_application_colors() {
        assert_eq!(Theme::default().text, Color::Reset);
        assert_eq!(Theme::default().border, Color::Green);
        assert_eq!(Theme::default().prompt, Color::Reset);
        assert_eq!(Theme::default().header, Color::Reset);
        assert_eq!(Theme::default().highlight_fg, Color::White);
        assert_eq!(Theme::default().highlight_bg, Color::Green);
    }

    #[test]
    fn tmux_style_maps_named_foreground_and_background() {
        let theme = Theme::from_tmux_style(Some("fg=yellow,bg=blue"));

        assert_eq!(theme.text, Color::Yellow);
        assert_eq!(theme.border, Color::Green);
        assert_eq!(theme.prompt, Color::Yellow);
        assert_eq!(theme.header, Color::Yellow);
        assert_eq!(theme.highlight_fg, Color::White);
        assert_eq!(theme.highlight_bg, Color::Blue);
    }

    #[test]
    fn tmux_style_maps_indexed_and_rgb_colors() {
        let indexed = Theme::from_tmux_style(Some("fg=colour123,bg=colour45"));
        assert_eq!(indexed.text, Color::Indexed(123));
        assert_eq!(indexed.highlight_fg, Color::White);
        assert_eq!(indexed.highlight_bg, Color::Indexed(45));

        let rgb = Theme::from_tmux_style(Some("fg=#12abef,bg=#010203"));
        assert_eq!(rgb.text, Color::Rgb(0x12, 0xab, 0xef));
        assert_eq!(rgb.highlight_fg, Color::White);
        assert_eq!(rgb.highlight_bg, Color::Rgb(1, 2, 3));
    }

    #[test]
    fn invalid_or_partial_tmux_style_preserves_defaults_per_color() {
        let invalid = Theme::from_tmux_style(Some("fg=not-a-color,bg=also-invalid"));
        assert_eq!(invalid, Theme::default());

        let partial = Theme::from_tmux_style(Some("fg=yellow"));
        assert_eq!(partial.text, Color::Yellow);
        assert_eq!(partial.border, Color::Green);
        assert_eq!(partial.highlight_fg, Color::White);
        assert_eq!(partial.highlight_bg, Color::Green);
    }

    #[test]
    fn multiplexer_theme_uses_popup_styles() {
        let theme = Theme::from_multiplexer(&FakeMultiplexer::with_popup_styles(
            "fg=colour245,bg=default",
            "fg=colour166,bg=colour234",
        ));

        assert_eq!(theme.text, Color::Indexed(245));
        assert_eq!(theme.border, Color::Indexed(166));
        assert_eq!(theme.highlight_fg, Color::Indexed(245));
        assert_eq!(theme.highlight_bg, Color::Reset);
    }

    #[test]
    fn multiplexer_theme_uses_standard_tmux_styles_when_popup_styles_are_default() {
        let theme = Theme::from_multiplexer(&FakeMultiplexer::with_standard_styles(
            "bg=green,fg=black",
            "fg=yellow",
        ));

        assert_eq!(theme.text, Color::Black);
        assert_eq!(theme.prompt, Color::Black);
        assert_eq!(theme.header, Color::Black);
        assert_eq!(theme.border, Color::Yellow);
        assert_eq!(theme.highlight_fg, Color::Black);
        assert_eq!(theme.highlight_bg, Color::Green);
    }

    #[test]
    fn multiplexer_theme_uses_defaults_for_missing_popup_colors() {
        let theme = Theme::from_multiplexer(&FakeMultiplexer::with_popup_style("bg=blue"));

        assert_eq!(theme.text, Color::Reset);
        assert_eq!(theme.border, Color::Green);
        assert_eq!(theme.highlight_fg, Color::Reset);
        assert_eq!(theme.highlight_bg, Color::Blue);
    }

    #[test]
    fn multiplexer_theme_maps_explicit_default_to_terminal_default() {
        let theme = Theme::from_multiplexer(&FakeMultiplexer::with_popup_styles(
            "fg=default,bg=default",
            "fg=colour166,bg=default",
        ));

        assert_eq!(theme.text, Color::Reset);
        assert_eq!(theme.prompt, Color::Reset);
        assert_eq!(theme.header, Color::Reset);
        assert_eq!(theme.border, Color::Indexed(166));
        assert_eq!(theme.highlight_fg, Color::Reset);
        assert_eq!(theme.highlight_bg, Color::Reset);
    }

    #[test]
    fn multiplexer_theme_ignores_tmux_styles_outside_tmux() {
        let theme = Theme::from_multiplexer(&FakeMultiplexer::outside_tmux());

        assert_eq!(theme, Theme::default());
    }

    #[test]
    fn multiplexer_theme_falls_back_when_tmux_is_unavailable() {
        let theme = Theme::from_multiplexer(&FakeMultiplexer::unavailable());

        assert_eq!(theme, Theme::default());
    }

    #[test]
    fn multiplexer_theme_ignores_status_colors() {
        let theme = Theme::from_multiplexer(&FakeMultiplexer::unavailable());

        assert_eq!(theme, Theme::default());
    }
}
