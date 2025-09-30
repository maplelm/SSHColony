use super::super::border::Border;
use std::fmt::Display;
use term::color::{Background, Foreground, Iso, Color};

pub struct Style {
    x: Measure,
    y: Measure,
    origin: Origin,
    width: Option<Measure>,
    height: Option<Measure>,
    border: Option<Border>,
    foreground: term::color::Foreground,
    background: term::color::Background,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            x: Measure::Cell(0),
            y: Measure::Cell(0),
            origin: Origin::TopLeft,
            width: None,
            height: None,
            border: None,
            foreground: Foreground::new(Color::Iso {
                color: Iso::White,
                bright: false,
            }),
            background: Background::new(Color::Iso {
                color: Iso::Black,
                bright: false,
            }),
        }
    }
}

// -------------
// Name: Measure
// Usage: stores a value with an associated unit
// -------------
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum Measure {
    Cell(u32),
    Percent(u8), // percent of totle terminal size
}

impl Measure {
    pub fn get(&self, max: usize) -> usize {
        match *self {
            Measure::Cell(val) => val as usize,
            Measure::Percent(val) => ((max as f32 / 100.0) * val as f32) as usize,
        }
    }

    pub fn get_raw(&self) -> usize {
        match *self {
            Measure::Cell(val) => val as usize,
            Measure::Percent(val) => val as usize,
        }
    }
}

#[derive(
    Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Origin {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

#[derive(
    Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Justify {
    Left,
    Right,
    Center,
}
