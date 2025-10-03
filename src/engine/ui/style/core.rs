#![deny(unused)]

use super::super::border::Border;
use serde::{Deserialize, Serialize};
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub size: Size,
    pub border: Option<Border>,
    pub justify: Justify,
    pub foreground: Option<term::color::Foreground>,
    pub background: Option<term::color::Background>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            size: Size {
                width: None,
                height: None
            },
            border: None,
            justify: Justify::Left,
            foreground: None,
            background: None,
        }
    }
}
