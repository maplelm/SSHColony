use std::fmt::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Mode(u8);
impl Mode {
    pub fn new(s: u8) -> Option<Self> {
        if (s > 9 && s < 22) || s == 6 || s == 26 || s > 29 {
            None
        } else {
            Some(Self(s))
        }
    }
}
impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m", self.0)
    }
}

pub const BOLD_MODE_ON: Mode = Mode(1);
pub const DIM_MODE_ON: Mode = Mode(2);
pub const ITALICS_MODE_ON: Mode = Mode(3);
pub const UNDERLINE_MODE_ON: Mode = Mode(4);
pub const INVERSE_MODE_ON: Mode = Mode(7);
pub const STRIKETHROUGH_MODE_ON: Mode = Mode(9);

pub const BOLD_MODE_OFF: Mode = Mode(22);
pub const DIM_MODE_OFF: Mode = Mode(22);
pub const ITALICS_MODE_OFF: Mode = Mode(23);
pub const UNDERLINE_MODE_OFF: Mode = Mode(24);
pub const INVERSE_MODE_OFF: Mode = Mode(27);
pub const STRIKETHROUGH_MODE_OFF: Mode = Mode(29);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Color {
    IsoForeground(u8),
    IsoBackground(u8),
    ExtendedForeground(u8),
    ExtendedBackground(u8),
    RgbForeground(u8, u8, u8),
    RgbBackground(u8, u8, u8),
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::IsoForeground(c) => write!(f, "\x1b[3{}m", c),
            Color::IsoBackground(c) => write!(f, "\x1b[4{}m", c),
            Color::ExtendedForeground(c) => write!(f, "\x1b[38;5;{}m", c),
            Color::ExtendedBackground(c) => write!(f, "\x1b[48;5;{}m", c),
            Color::RgbForeground(r, g, b) => write!(f, "\x1b[38;2;{};{};{}m", r, g, b),
            Color::RgbBackground(r, g, b) => write!(f, "\x1b[48;2;{};{};{}m", r, g, b),
        }
    }
}

pub const ISO_BLACK_FOREGROUND: Color = Color::IsoForeground(0);
pub const ISO_BLACK_BACKGROUND: Color = Color::IsoBackground(0);
pub const ISO_RED_FOREGROUND: Color = Color::IsoForeground(1);
pub const ISO_RED_BACKGROUND: Color = Color::IsoBackground(1);
pub const ISO_GREEN_FOREGROUND: Color = Color::IsoForeground(2);
pub const ISO_GREEN_BACKGROUND: Color = Color::IsoBackground(2);
pub const ISO_YELLOW_FOREGROUND: Color = Color::IsoForeground(3);
pub const ISO_YELLOW_BACKGROUND: Color = Color::IsoBackground(3);
pub const ISO_BLUE_FOREGROUND: Color = Color::IsoForeground(4);
pub const ISO_BLUE_BACKGROUND: Color = Color::IsoBackground(4);
pub const ISO_MAGENTA_FOREGROUND: Color = Color::IsoForeground(5);
pub const ISO_MAGENTA_BACKGROUND: Color = Color::IsoBackground(5);
pub const ISO_CYAN_FOREGROUND: Color = Color::IsoForeground(6);
pub const ISO_CYAN_BACKGROUND: Color = Color::IsoBackground(6);
pub const ISO_WHITE_FOREGROUND: Color = Color::IsoForeground(7);
pub const ISO_WHITE_BACKGROUND: Color = Color::IsoBackground(7);

pub const COLOR_RESET: &'static str = "\x1b[0m";