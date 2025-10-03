//#![deny(unused)]

use super::Canvas;
use crate::engine::types::Position3D;
use crate::engine::ui::style::Justify;
use crate::engine::ui::{Border, style::Measure};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::time::{Duration, Instant};
use term::color::{Background, Foreground};

////////////////////////////////////////////////////////////////////////////////////////////////
// Sprites /////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteBase {
    pub symbol: char,
    pub fg: Option<Foreground>,
    pub bg: Option<Background>,
    cached: Option<String>,
}
impl SpriteBase {
    pub fn new(sym: char, fg: Option<Foreground>, bg: Option<Background>) -> Self {
        Self {
            symbol: sym,
            fg: fg,
            bg: bg,
            cached: None,
        }
    }
    pub fn as_str(&mut self) -> &str {
        if self.cached.is_some() {
            self.cached.as_ref().unwrap()
        } else {
            let mut s: String;
            if let Some(fg) = self.fg.as_ref() {
                s = fg.to_ansi();
                s.push(self.symbol);
            } else {
                s = String::from(self.symbol);
            }
            if let Some(bg) = self.bg.as_ref() {
                s.push_str(&bg.to_ansi());
            }
            self.cached = Some(s);
            self.cached.as_ref().unwrap()
        }
    }
}

////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StaticSprite {
    pub pos: Position3D<i32>,
    pub base: SpriteBase,
}

impl Display for StaticSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            if let Some(fg) = &self.base.fg {
                fg.to_ansi()
            } else {
                "".to_string()
            },
            if let Some(bg) = &self.base.bg {
                bg.to_ansi()
            } else {
                "".to_string()
            },
            self.base.symbol,
        )
    }
}

impl StaticSprite {
    pub fn as_str(&mut self) -> &str {
        self.base.as_str()
    }
}

////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicSprite {
    pub pos: Position3D<i32>,
    pub sprite_sheet: Vec<SpriteBase>,
    pub tick: Duration,
    #[serde(skip, default = "DynamicSprite::default_cursor_pos")]
    pub cursor: usize,
    #[serde(skip, default = "Instant::now")]
    pub last_tick: Instant,
}

impl DynamicSprite {
    fn default_cursor_pos() -> usize {
        0
    }
    pub fn new(pos: Position3D<i32>, ss: Vec<SpriteBase>, tick: Duration) -> Self {
        Self {
            pos: pos,
            sprite_sheet: ss,
            tick: tick,
            cursor: 0,
            last_tick: Instant::now(),
        }
    }

    pub fn as_str(&mut self) -> &str {
        self.sprite_sheet[self.cursor].as_str()
    }

    pub fn update(&mut self) -> bool {
        if self.last_tick.elapsed() >= self.tick {
            self.cursor = (self.cursor + 1) % self.sprite_sheet.len();
            self.last_tick = Instant::now();
            true
        } else {
            false
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Text ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextBase {
    text: String,
    justify: Justify,
    width: Option<Measure>,
    height: Option<Measure>,
    border: Option<Border>,
    fg: Option<Foreground>,
    bg: Option<Background>,
    cached: Option<String>,
}

impl TextBase {
    pub fn new(
        text: String,
        justify: Justify,
        width: Option<Measure>,
        height: Option<Measure>,
        border: Option<Border>,
        fg: Option<Foreground>,
        bg: Option<Background>,
    ) -> Self {
        Self {
            text: text,
            justify: justify,
            width: width,
            height: height,
            border: border,
            fg: fg,
            bg: bg,
            cached: None,
        }
    }

    // Does not consider the Height measurement
    pub fn as_str(&mut self, canvas: &Canvas) -> &str {
        if self.cached.is_some() {
            self.cached.as_ref().unwrap()
        } else {
            let width = self.width(canvas);
            let color = TextBase::color_string(&self.fg, &self.bg);
            let mut output = String::from(&color);

            self.top_border(width, &mut output);
            let iter = self.top_padding(width, &mut output, &color);
            let iter = self.lines(width, &mut output, &color, iter);
            self.bottom_padding(width, &mut output, &color, iter);
            self.bottom_border(width, &mut output, &color);
            self.cached = Some(output);
            self.cached.as_ref().unwrap()
        }
    }

    pub fn fg(&self) -> Option<&Foreground> {
        self.fg.as_ref()
    }

    pub fn bg(&self) -> Option<&Background> {
        self.bg.as_ref()
    }

    fn color_string(fg: &Option<Foreground>, bg: &Option<Background>) -> String {
        let mut output: String = String::new();
        if let Some(fg) = fg {
            output.push_str(&fg.to_ansi());
        }
        if let Some(bg) = bg {
            output.push_str(&bg.to_ansi());
        }
        return output;
    }

    fn get_justfy_gaps(&self, line_width: usize, total_width: usize) -> (usize, usize) {
        let mut extra = total_width;
        if let Some(b) = self.border.as_ref() {
            extra -= line_width + b.get_pad_left() + b.get_pad_right() + b.width();
        } else {
            extra -= line_width;
        }
        match self.justify {
            Justify::Center => {
                if extra % 2 == 0 {
                    (extra / 2, extra / 2)
                } else {
                    (extra / 2, extra / 2 + 1)
                }
            }
            Justify::Left => (0, extra),
            Justify::Right => (extra, 0),
        }
    }

    pub fn width(&self, canvas: &Canvas) -> usize {
        if let Some(w) = self.width.as_ref() {
            w.get(canvas.width)
        } else {
            if let Some(b) = self.border.as_ref() {
                b.get_pad_left() + self.longest_line() + b.get_pad_right() + b.width()
            } else {
                self.longest_line()
            }
        }
    }

    pub fn longest_line(&self) -> usize {
        if self.text.split("\n").count() < 2 {
            self.text.len()
        } else {
            let mut max: usize = 0;
            for line in self.text.split("\n") {
                if max < line.len() {
                    max = line.len();
                }
            }
            max
        }
    }

    fn top_border(&self, width: usize, output: &mut String) {
        if let Some(b) = self.border.as_ref()
            && !b.is_top_none()
        {
            for i in 0..width {
                output.push(b.get_top(i).unwrap());
            }
            output.push('\n');
        }
    }

    fn top_padding(&self, width: usize, output: &mut String, prefix: &str) -> usize {
        if let Some(b) = self.border.as_ref()
            && b.get_pad_top() > 0
        {
            let mut iter: usize = 0;
            let mut spacing = String::new();
            for _ in 0..width - b.width() {
                spacing.push(' ');
            }
            for _ in 0..b.get_pad_top() {
                output.push_str(prefix);

                if !b.is_left_none() {
                    output.push(b.get_left(iter).unwrap());
                }

                output.push_str(spacing.as_str());

                if !b.is_right_none() {
                    output.push(b.get_right(iter).unwrap());
                }

                output.push('\n');
                iter += 1;
            }
            return iter;
        }
        return 0;
    }

    fn lines(&self, width: usize, output: &mut String, prefix: &str, mut iter: usize) -> usize {
        for line in self.text.split("\n") {
            output.push_str(prefix);
            if let Some(b) = self.border.as_ref() {
                if let Some(c) = b.get_left(iter) {
                    output.push(c);
                }
                let (jl, jr) = self.get_justfy_gaps(line.len(), width);
                for _ in 0..(b.get_pad_left() + jl) {
                    output.push(' ');
                }
                if let Some(s) = self.truncate_line(line, width) {
                    output.push_str(&s);
                } else {
                    output.push_str(line);
                }
                for _ in 0..(b.get_pad_right() + jr) {
                    output.push(' ');
                }
                if let Some(c) = b.get_right(iter) {
                    output.push(c);
                }
                output.push('\n');
                iter += 1;
            }
        }
        iter
    }

    fn bottom_padding(&self, width: usize, output: &mut String, prefix: &str, mut iter: usize) {
        if let Some(b) = self.border.as_ref() {
            let mut spacing = String::new();
            for _ in 0..width - b.width() {
                spacing.push(' ');
            }
            for _ in 0..b.get_pad_bottom() {
                output.push_str(prefix);
                if let Some(c) = b.get_left(iter) {
                    output.push(c);
                }
                output.push_str(&spacing);
                if let Some(c) = b.get_right(iter) {
                    output.push(c);
                }
                output.push('\n');
                iter += 1;
            }
        }
    }
    fn bottom_border(&self, width: usize, output: &mut String, prefix: &str) {
        if let Some(b) = self.border.as_ref()
            && !b.is_bottom_none()
        {
            output.push_str(prefix);
            for i in 0..width {
                output.push(b.get_bottom(i).unwrap());
            }
        }
    }

    fn truncate_line(&self, line: &str, width: usize) -> Option<String> {
        let mut max_len: usize = width;
        if let Some(b) = self.border.as_ref() {
            max_len -= b.width() + b.get_pad_left() + b.get_pad_right();
        }
        if line.len() > max_len {
            if let Some(l) = line.get(
                0..line
                    .len()
                    .wrapping_sub(line.len().wrapping_sub(max_len) + 3),
            ) {
                let mut s = String::from(l.to_string());
                s.push_str("...");
                Some(s)
            } else {
                if let Some(l) = line.get(0..line.len().wrapping_sub(max_len)) {
                    Some(l.to_string())
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Static Text //////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StaticText {
    pub pos: Position3D<i32>,
    pub base: TextBase,
}

impl StaticText {
    pub fn as_str(&mut self, canvas: &Canvas) -> &str {
        self.base.as_str(canvas)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Dynamic Text /////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicText {
    pub pos: Position3D<i32>,
    pub text_sheet: Vec<TextBase>,
    pub tick: Duration,
    #[serde(skip, default = "DynamicText::default_cursor_pos")]
    pub cursor: usize,
    #[serde(skip, default = "Instant::now")]
    pub last_tick: Instant,
}

impl DynamicText {
    pub fn default_cursor_pos() -> usize {
        0
    }

    pub fn as_str(&mut self, canvas: &Canvas) -> &str {
        self.text_sheet[self.cursor].as_str(canvas)
    }

    pub fn update(&mut self) -> bool {
        if self.last_tick.elapsed() >= self.tick {
            self.cursor = (self.cursor + 1) % self.text_sheet.len();
            self.last_tick = Instant::now();
            true // did change
        } else {
            false // no change
        }
    }
}
