#![deny(unused)]

use std::time::{Duration, Instant};
use std::fmt::Display;
use term::color::{Background, Foreground};
use super::{Layer, Canvas};
use crate::engine::types::Position;
use crate::engine::ui::style::Justify;
use crate::engine::ui::{Border, style::Measure};

////////////////////////////////////////////////////////////////////////////////////////////////
// Sprites /////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct SpriteBase {
    pub symbol: char,
    pub layer: Layer,
    pub fg: Option<Foreground>,
    pub bg: Option<Background>
}
impl SpriteBase {
    pub fn to_string(&self) -> String {
        let mut s: String;
        if let Some(fg) = self.fg.as_ref() { 
            s = fg.to_ansi();
            s.push(self.symbol);
        } else {
            s = String::from(self.symbol);
        }
        if let Some(bg) =self.bg.as_ref() {
            s.push_str(&bg.to_ansi());
        }
        return s;
    }
}

////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct StaticSprite {
    pub pos: Position<i32>,
    pub base: SpriteBase
}

impl Display for StaticSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}", 
            if let Some(fg) = &self.base.fg {fg.to_ansi()} else {"".to_string()},
            if let Some(bg) = &self.base.bg {bg.to_ansi()} else {"".to_string()},
            self.base.symbol,
        )
    }
}

impl StaticSprite {
    pub fn to_string(&self) -> String {
        self.base.to_string()
    }
}


////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct DynamicSprite {
    pub pos: Position<i32>,
    pub sprite_sheet: Vec<SpriteBase>,
    pub tick: Duration,
    pub cursor: usize,
    pub last_tick: Instant
}

impl DynamicSprite {
    pub fn new(pos: Position<i32>, ss: Vec<SpriteBase>, tick: Duration) -> Self {
        Self {
            pos: pos,
            sprite_sheet: ss,
            tick: tick,
            cursor: 0,
            last_tick: Instant::now()
        }
    }

    pub fn to_string(&self) -> String {
        self.sprite_sheet[self.cursor].to_string()
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

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Text /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
 

#[derive(Clone, Debug)]
pub struct TextBase {
    pub text: String,
    pub layer: Layer,
    pub justify: Justify,
    pub width: Option<Measure>,
    pub height: Option<Measure>,
    pub border: Option<Border>,
    pub fg: Option<Foreground>,
    pub bg: Option<Background>,
}

impl TextBase {
    // Does not consider the Height measurement
    pub fn to_string(&self, canvas: &Canvas) -> String {
        let width = self.width(canvas);
        let fg = if let Some(fg) = self.fg.as_ref() { fg.to_ansi()} else {"".to_string()};
        let bg = if let Some(bg) = self.bg.as_ref() { bg.to_ansi()} else {"".to_string()};
        let mut prefix = fg;
        prefix.push_str(&bg);
        let mut output = String::from(&prefix);

        self.top_border(width, &mut output);
        let iter = self.top_padding(width, &mut output, &prefix);
        let iter = self.lines(width, &mut output, &prefix, iter);
        self.bottom_padding(width, &mut output, &prefix, iter);
        self.bottom_border(width, &mut output, &prefix);
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
                    (extra/2,extra/2)
                } else {
                    (extra/2,extra/2+1)
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
        if let Some(b) = self.border.as_ref() && !b.is_top_none() {
            for i in 0..width {
                output.push(b.get_top(i).unwrap());
            }
            output.push('\n');
        }
    }

    fn top_padding(&self, width: usize, output: &mut String, prefix: &str) -> usize {
        if let Some(b) = self.border.as_ref() && b.get_pad_top() > 0 {
            let mut iter: usize = 0;
            let mut spacing = String::new();
            for _ in 0..width-b.width() {
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
                for _ in 0..b.get_pad_left() {
                    output.push(' ');
                }
                let (jl, jr) = self.get_justfy_gaps(line.len(), width);
                for _ in 0..jl {
                    output.push(' ');
                }
                if let Some(s) = self.truncate_line(line, width) {
                    output.push_str(&s);
                } else {
                    output.push_str(line);
                }
                for _ in 0..jr {
                    output.push(' ');
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
            for _ in 0..width-b.width() {
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
        if let Some(b) = self.border.as_ref() && !b.is_bottom_none() {
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
            if let Some(l) = line.get(0..line.len().wrapping_sub(line.len().wrapping_sub(max_len)+3)) { 
                let mut s = String::from(l.to_string());
                s.push_str("...");
                Some(s)
            } else { 
                if let Some(l) = line.get(0..line.len().wrapping_sub(max_len)) { Some(l.to_string())} else {None}
            }
        } else {
            None
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Static Text //////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct StaticText {
    pub pos: Position<i32>,
    pub base: TextBase,
}

impl StaticText {
    pub fn to_string(&self, canvas: &Canvas) -> String {
        self.base.to_string(canvas)
    }
}


/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Dynamic Text /////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct DynamicText {
    pub pos: Position<i32>,
    pub text_sheet: Vec<TextBase>,
    pub tick: Duration,
    pub cursor: usize,
    pub last_tick: Instant,
}

impl DynamicText {
    pub fn default_cursor_pos() -> usize {
        0
    }

    pub fn to_string(&self, canvas: &Canvas) -> String {
        self.text_sheet[self.cursor].to_string(canvas)
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