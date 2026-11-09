use ratatui::{Terminal, backend::TestBackend, style::Color};
use senkai_presentation::tui::theme::Theme;
use senkai_presentation::tui::widgets::loading::LoadingIndicator;

mod tests {
    use super::*;

    #[test]
    fn loading_indicator_uses_injected_theme() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let theme = Theme {
            text: Color::Green,
            border: Color::Cyan,
            prompt: Color::Cyan,
            header: Color::Yellow,
            highlight_fg: Color::Black,
            highlight_bg: Color::Cyan,
            highlight_modifier: ratatui::style::Modifier::BOLD,
        };
        let mut indicator = LoadingIndicator::with_theme("Loading", theme);

        terminal
            .draw(|frame| indicator.render(frame, frame.area()))
            .unwrap();

        let has_green = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .any(|cell| cell.fg == Color::Green);
        assert!(has_green);
    }
}
