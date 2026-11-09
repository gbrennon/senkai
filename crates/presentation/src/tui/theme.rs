use ratatui::style::{Color, Modifier};
use senkai_core::ports::multiplexer_port::MultiplexerPort;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    pub text: Color,
    pub border: Color,
    pub prompt: Color,
    pub header: Color,
    pub highlight_fg: Color,
    pub highlight_bg: Color,
    pub highlight_modifier: Modifier,
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
    pub fn from_multiplexer(terminal: &dyn MultiplexerPort) -> Self {
        let status_left = tmux_option(terminal, "status-left");
        if let Some(theme) = status_left.as_deref().and_then(theme_from_style) {
            return theme;
        }

        tmux_option(terminal, "status-style")
            .as_deref()
            .and_then(theme_from_style)
            .unwrap_or_default()
    }

    pub fn from_tmux_style(style: Option<&str>) -> Self {
        style.and_then(theme_from_style).unwrap_or_default()
    }
}

fn tmux_option(terminal: &dyn MultiplexerPort, option: &str) -> Option<String> {
    terminal
        .run(&["show-options", "-v", option])
        .ok()
        .and_then(|(status, output, _)| (status == 0).then_some(output))
}

fn theme_from_style(style: &str) -> Option<Theme> {
    let foreground = style_color(style, "fg")?;
    let background = style_color(style, "bg")?;

    Some(Theme {
        text: foreground,
        border: foreground,
        prompt: foreground,
        header: foreground,
        highlight_fg: background,
        highlight_bg: foreground,
        ..Theme::default()
    })
}

fn style_color(style: &str, key: &str) -> Option<Color> {
    style.split(',').find_map(|part| {
        let part = part.trim().strip_prefix("#[").unwrap_or(part.trim());
        let (part_key, value) = part.split_once('=')?;
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
