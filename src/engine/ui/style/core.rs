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

use super::super::border::Border;
use super::types::*;
use my_term::color::{Background, Foreground};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub size: Size,
    pub border: Option<Border>,
    pub alignment: Alignment,
    pub color: Coloring,
}

impl Style {
    pub fn from(
        size: Size,
        border: Option<Border>,
        alignment: Alignment,
        justify: Justify,
        align: Align,
        color: Coloring,
    ) -> Self {
        Self {
            size,
            border,
            alignment,
            color,
        }
    }

    pub fn set_width(mut self, w: Measure) -> Self {
        self.size.width = Some(w);
        self
    }

    pub fn set_height(mut self, h: Measure) -> Self {
        self.size.height = Some(h);
        self
    }

    pub fn set_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn set_border(mut self, b: Border) -> Self {
        self.border = Some(b);
        self
    }
    pub fn set_justify(mut self, j: Justify) -> Self {
        self.alignment.justify = j;
        self
    }

    pub fn set_align(mut self, a: Align) -> Self {
        self.alignment.align = a;
        self
    }

    pub fn set_fg(mut self, fg: Foreground) -> Self {
        self.color.foreground = fg;
        self
    }

    pub fn set_bg(mut self, bg: Background) -> Self {
        self.color.background = bg;
        self
    }

    pub fn justify(&self) -> Justify {
        self.alignment.justify
    }

    pub fn align(&self) -> Align {
        self.alignment.align
    }

    pub fn fg(&self) -> &Foreground {
        &self.color.foreground
    }

    pub fn bg(&self) -> &Background {
        &self.color.background
    }

    pub fn height(&self) -> Option<&Measure> {
        self.size.height.as_ref()
    }

    pub fn width(&self) -> Option<&Measure> {
        self.size.width.as_ref()
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            size: Size::default(),
            border: None,
            alignment: Alignment::default(),
            color: Coloring::default(),
        }
    }
}
