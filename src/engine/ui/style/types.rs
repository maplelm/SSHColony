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

use std::fmt::Display;

use my_term::color::{Background, Foreground, WHITE, BLACK};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coloring {
    pub foreground: Foreground,
    pub background: Background,
}

impl Coloring {
    pub fn new(fg: impl Into<u8>, bg: impl Into<u8>) -> Self {
        Self {
            foreground: Foreground::new(fg.into()),
            background: Background::new(bg.into()),
        }
    }
    pub fn set_fg(mut self, fg: Foreground) -> Self {
        self.foreground = fg;
        self
    }

    pub fn set_bg(mut self, bg: Background) -> Self {
        self.background = bg;
        self
    }
}

impl Default for Coloring {
    fn default() -> Self {
        Self {
            foreground: Foreground::new(WHITE),
            background: Background::new(BLACK),
        }
    }
}

impl Display for Coloring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.background, self.foreground)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Measure {
    Cell(u32),
    Percent(u8),
}

impl Measure {
    pub fn get(&self, max: usize) -> usize {
        match *self {
            Measure::Cell(val) => val as usize,
            Measure::Percent(val) => ((max as f64 / 100.0) * val as f64) as usize,
        }
    }

    pub fn get_raw(&self) -> usize {
        match *self {
            Measure::Cell(val) => val as usize,
            Measure::Percent(val) => val as usize,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alignment {
    pub justify: Justify,
    pub align: Align,
}

impl Alignment {
    pub fn centered() -> Self {
        Self {
            justify: Justify::Center,
            align: Align::Center,
        }
    }

    pub fn top_right() -> Self {
        Self {
            justify: Justify::Right,
            align: Align::Top,
        }
    }

    pub fn bot_left() -> Self {
        Self {
            justify: Justify::Left,
            align: Align::Bottom,
        }
    }

    pub fn bot_right() -> Self {
        Self {
            justify: Justify::Right,
            align: Align::Bottom,
        }
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Self {
            justify: Justify::Left,
            align: Align::Top,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Justify {
    Left,
    Right,
    Center,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Align {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Size {
    pub width: Option<Measure>,
    pub height: Option<Measure>,
}

impl Size {
    pub fn new(w: Option<Measure>, h: Option<Measure>) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    pub fn w_only(w: Measure) -> Self {
        Self {
            width: Some(w),
            height: None,
        }
    }

    pub fn h_only(h: Measure) -> Self {
        Self {
            width: None,
            height: Some(h),
        }
    }

    pub fn square(size: Measure) -> Self {
        Self {
            width: Some(size),
            height: Some(size),
        }
    }

    pub fn rect(width: Measure, height: Measure) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::engine::consts::DEFAULT_CANVAS;
    use my_term::term_size;
    #[test]
    fn measure_test() {
        let mut canvas = DEFAULT_CANVAS;
        if let Some(size) = term_size() {
            canvas.width = size.0 as usize;
            canvas.height = size.1 as usize;
        }

        let x = Measure::Cell(40);
        assert_eq!(x.get(canvas.height), 40);
        let x = Measure::Cell(60);
        assert_eq!(x.get(canvas.height), 60);
        let x = Measure::Percent(100);
        assert_eq!(x.get(canvas.height), canvas.height);
        let x = Measure::Percent(50);
        assert_eq!(x.get(canvas.height), canvas.height / 2);
        let x = Measure::Percent(25);
        assert_eq!(x.get(canvas.height), canvas.height / 4);
    }
}
