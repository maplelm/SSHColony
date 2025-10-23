/*
Copyright 2025 Luke Maple

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
you may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

#![deny(unused)]

use super::super::border::Border;
use super::types::*;
use serde::{Deserialize, Serialize};
use my_term::color::{Background, Foreground};

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
    #[allow(unused)]
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
