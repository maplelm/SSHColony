#![deny(unused)]

use super::super::border::Border;
use super::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub size: Size,
    pub border: Option<Border>,
    pub justify: Justify,
    pub align: Align,
    pub foreground: Option<term::color::Foreground>,
    pub background: Option<term::color::Background>,
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
