use my_term::color::{BLACK, Background, Foreground, WHITE};
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Write;

use crate::engine::render::Text;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Character {
    fg: Foreground,
    bg: Background,
    sym: char,
}

impl Character {
    pub fn new(c: char, fg: impl Into<u8>, bg: impl Into<u8>) -> Self {
        Self {
            fg: Foreground::new(fg.into()),
            bg: Background::new(bg.into()),
            sym: c,
        }
    }
    pub fn set_fg(mut self, fg: u8) -> Self {
        self.fg = Foreground::new(fg);
        self
    }

    pub fn set_bg(mut self, bg: u8) -> Self {
        self.bg = Background::new(bg);
        self
    }

    pub fn set_sym(mut self, c: char) -> Self {
        self.sym = c;
        self
    }

    pub fn same_colors(&self, other: &Character) -> bool {
        self.fg == other.fg && self.bg == other.bg
    }

    pub fn as_char(&self) -> char {
        self.sym
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            fg: Foreground::new(WHITE),
            bg: Background::new(BLACK),
            sym: ' ',
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.fg, self.bg, self.sym)
    }
}

impl Into<Text> for &[Character] {
    fn into(self) -> Text {
        let mut output = Text::new();
        for c in self {
            output.push(c.clone());
        }
        output
    }
}

pub trait PushChar {
    fn push_char(&mut self, c: &Character);
}

impl PushChar for String {
    fn push_char(&mut self, c: &Character) {
        write!(self, "{}{}{}", c.fg, c.bg, c.sym);
    }
}
