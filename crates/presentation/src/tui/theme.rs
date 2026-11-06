use ratatui::style::{Color, Modifier};

/// Single shared color palette for every interactive TUI surface.
///
/// Both `sk` run directly and `sk` invoked through the tmux popup render
/// through this same palette, so direct and tmux launches look identical.
pub(crate) const TEXT: Color = Color::White;
pub(crate) const BORDER: Color = Color::Cyan;
pub(crate) const PROMPT: Color = Color::Cyan;
pub(crate) const HEADER: Color = Color::Yellow;
pub(crate) const HIGHLIGHT_FG: Color = Color::Black;
pub(crate) const HIGHLIGHT_BG: Color = Color::Cyan;
pub(crate) const HIGHLIGHT_MODIFIER: Modifier = Modifier::BOLD;
