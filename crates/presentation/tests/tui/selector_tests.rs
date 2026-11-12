use std::sync::{Arc, Mutex};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Frame, Terminal, backend::TestBackend, buffer::Buffer};
use senkai_core::error::{Error, Result};
use senkai_core::selector::Selector;
use senkai_presentation::tui::RatatuiSelector;
use senkai_presentation::tui::terminal_backend::TerminalBackend;
use senkai_presentation::tui::theme::Theme;

struct FakeTerminal {
    last_buffer: Arc<Mutex<Option<Buffer>>>,
}

impl FakeTerminal {
    fn new() -> Self {
        Self {
            last_buffer: Arc::new(Mutex::new(None)),
        }
    }
}

impl TerminalBackend for FakeTerminal {
    type Term = TestBackend;

    fn setup(&self) -> Result<Terminal<Self::Term>> {
        Terminal::new(TestBackend::new(40, 10))
            .map_err(|error| Error::new(format!("failed to create test terminal: {error}")))
    }

    fn draw_frame(
        &self,
        terminal: &mut Terminal<Self::Term>,
        render: impl FnOnce(&mut Frame),
    ) -> Result<()> {
        terminal
            .draw(render)
            .map(|_| ())
            .map_err(|error| Error::new(format!("failed to draw test terminal: {error}")))
    }

    fn read_event(&self) -> Result<Event> {
        Ok(Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)))
    }

    fn restore(&self, terminal: &mut Terminal<Self::Term>) -> Result<()> {
        *self.last_buffer.lock().unwrap() = Some(terminal.backend().buffer().clone());
        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn selector_uses_injected_theme_styles() {
        let backend = FakeTerminal::new();
        let theme = Theme {
            text: ratatui::style::Color::Red,
            border: ratatui::style::Color::Green,
            prompt: ratatui::style::Color::Blue,
            header: ratatui::style::Color::Yellow,
            highlight_fg: ratatui::style::Color::White,
            highlight_bg: ratatui::style::Color::Magenta,
            highlight_modifier: ratatui::style::Modifier::BOLD,
        };
        let mut selector = Selector::new(vec!["alpha".into(), "beta".into()], false);
        RatatuiSelector::run_with_backend(
            &backend,
            &mut selector,
            &[],
            "Worktree> ",
            "hint",
            &theme,
        )
        .unwrap();

        let buffer = backend.last_buffer.lock().unwrap().clone().unwrap();
        let selected = buffer[(1, 5)].style();
        assert_eq!(selected.fg, Some(ratatui::style::Color::White));
        assert_eq!(selected.bg, Some(ratatui::style::Color::Magenta));
        let ordinary = buffer[(1, 6)].style();
        assert_eq!(ordinary.fg, Some(ratatui::style::Color::Red));
    }
}
