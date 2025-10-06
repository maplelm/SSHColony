#![deny(unused)]

use super::super::border::Border;
use super::types::*;
use serde::{Deserialize, Serialize};
use term::color::{Background, Foreground};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub size: Size,
    pub border: Option<Border>,
    pub justify: Justify,
    pub align: Align,
    pub foreground: Option<Foreground>,
    pub background: Option<Background>,
}

impl Style {
    pub fn from(
        width: Option<Measure>,
        height: Option<Measure>,
        border: Option<Border>,
        justify: Justify,
        align: Align,
        fg: Option<Foreground>,
        bg: Option<Background>,
    ) -> Self {
        Self {
            size: Size {
                width: width,
                height: height,
            },
            border: border,
            justify: justify,
            align: align,
            foreground: fg,
            background: bg,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            size: Size {
                width: None,
                height: None,
            },
            border: None,
            justify: Justify::Left,
            align: Align::Center,
            foreground: None,
            background: None,
        }
    }
}
