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

//#![deny(unused)]

use super::Canvas;
use crate::engine::types::Position3D;
use crate::engine::ui::style::{Align, Justify};
use crate::engine::ui::{Border, style::Measure};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Write;
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
    align: Align,
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
        align: Align,
        width: Option<Measure>,
        height: Option<Measure>,
        border: Option<Border>,
        fg: Option<Foreground>,
        bg: Option<Background>,
    ) -> Self {
        Self {
            text: text,
            justify: justify,
            align: align,
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
            // Return Cached value, nothing has changed
            self.cached.as_ref().unwrap()
        } else {
            self.generate_str_cache(canvas);
            self.cached.as_ref().unwrap().as_str()
        }
    }

    fn generate_str_cache(&mut self, canvas: &Canvas) {
        let w = self.width(canvas); // With of text
        let h = self.height(canvas); // Height of Text
        let lc = self.get_line_count(canvas); // Line Count
        let bw = self.border_width();
        let bh = self.border_height();
        let (top_extra, bot_extra) = self.get_align_gaps(h, bh, lc);
        let color = TextBase::color_string(&self.fg, &self.bg);
        let mut output = String::from(&color);
        self.top_border(w, &mut output);
        let i = self.top_padding(w, h, top_extra, lc, &mut output, &color);
        let i = self.lines(w, h, top_extra, bot_extra, lc, &mut output, &color, i);
        self.bottom_padding(w, h, bot_extra, lc, &mut output, &color, i);
        self.bottom_border(w, &mut output, &color);
        self.cached = Some(output);
    }

    pub fn get_line_count(&self, canvas: &Canvas) -> usize {
        let mut size = self.text.split("\n").count();
        if self.height.is_none() {
            return size;
        }
        let h = self.height.unwrap().get(canvas.height);
        if self.border.is_none() {
            return if size > h { h } else { size };
        }
        let b = self.border.as_ref().unwrap();
        let b = b.get_pad_top() + b.get_pad_bottom() + b.height();
        return if size > h - b { h - b } else { size };
    }

    fn get_align_gaps(&self, height: usize, bh: usize, lc: usize) -> (usize, usize) {
        let extra = if height > (bh + lc) {
            height - (bh + lc)
        } else {
            0
        };
        #[cfg(test)]
        {
            println!("get_align_gaps():");
            println!("\tHeight: {height}");
            println!("\tbh: {bh}");
            println!("\tlc: {lc}");
            println!("\tExtra: {}", extra);
        }

        match self.align {
            Align::Top => (0, extra),
            Align::Center => (
                if extra <= 3 {
                    extra / 2
                } else if extra % 2 == 1 {
                    extra / 2 + 1
                } else {
                    extra / 2
                },
                extra / 2,
            ),
            Align::Bottom => (extra, 0),
        }
    }

    fn border_width(&self) -> usize {
        if let Some(b) = self.border.as_ref() {
            b.width() + b.get_pad_right() + b.get_pad_left()
        } else {
            0
        }
    }

    fn border_height(&self) -> usize {
        if let Some(b) = self.border.as_ref() {
            b.height() + b.get_pad_top() + b.get_pad_bottom()
        } else {
            0
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

    fn get_justify_gaps(&self, line_width: usize, total_width: usize) -> (usize, usize) {
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

    pub fn height(&self, canvas: &Canvas) -> usize {
        if let Some(h) = self.height.as_ref() {
            h.get(canvas.height)
        } else {
            if let Some(b) = self.border.as_ref() {
                b.get_pad_top() + b.get_pad_bottom() + b.height() + self.text.split("\n").count()
            } else {
                self.text.split("\n").count()
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
                output.push(
                    b.get_top(i)
                        .unwrap_or(crate::engine::ui::style::LIGHT_BLOCK),
                );
            }
            output.push('\n');
        }
    }

    fn top_padding(
        &self,
        width: usize,
        height: usize,
        extra: usize,
        line_count: usize,
        output: &mut String,
        prefix: &str,
    ) -> usize {
        if let Some(b) = self.border.as_ref()
            && b.get_pad_top() > 0
        {
            let mut iter: usize = 0;
            let mut spacing = String::new();
            for _ in 0..width - b.width() {
                spacing.push(' ');
            }
            for _ in 0..(b.get_pad_top() + extra) {
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

    fn lines(
        &self,
        width: usize,
        height: usize,
        extra_top_pad: usize,
        extra_bot_pad: usize,
        line_count: usize,
        output: &mut String,
        prefix: &str,
        mut iter: usize,
    ) -> usize {
        for (i, mut line) in self.text.split("\n").enumerate() {
            output.push_str(prefix);
            if let Some(b) = self.border.as_ref() {
                if let Some(c) = b.get_left(iter) {
                    output.push(c);
                }
                if (if b.is_top_none() { 0 } else { 1 } + b.get_pad_top() + extra_top_pad + i + 1)
                    == height
                        - (b.get_pad_bottom()
                            + extra_bot_pad
                            + if b.is_bottom_none() { 0 } else { 1 })
                    && i + 1 < line_count
                {
                    line = match line.len() {
                        1 => ".",
                        2 => "..",
                        _ => "...",
                    };
                }
                let (jl, jr) = self.get_justify_gaps(TextBase::string_width(line), width);
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
                if (if b.is_top_none() { 0 } else { 1 } + b.get_pad_top() + extra_top_pad + i + 1)
                    == height
                        - (b.get_pad_bottom()
                            + extra_bot_pad
                            + if b.is_bottom_none() { 0 } else { 1 })
                    && i + 1 < line_count
                {
                    break;
                }
            }
        }
        iter
    }

    // trys to ignore ansi color codes
    fn string_width(s: &str) -> usize {
        //let mut base = s.len();
        let mut count: usize = 0;
        let mut add = true;
        for c in s.chars() {
            if c == '\x1b' {
                add = false;
            } else if c == 'm' && !add {
                add = true;
            } else if add {
                count += 1;
            }
        }
        count
    }

    fn bottom_padding(
        &self,
        width: usize,
        height: usize,
        extra: usize,
        line_count: usize,
        output: &mut String,
        prefix: &str,
        mut iter: usize,
    ) {
        if let Some(b) = self.border.as_ref() {
            let mut spacing = String::new();
            for _ in 0..width - b.width() {
                spacing.push(' ');
            }
            for _ in 0..(b.get_pad_bottom() + extra) {
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
                output.push(
                    b.get_bottom(i)
                        .unwrap_or(crate::engine::ui::style::LIGHT_BLOCK),
                );
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

    pub fn line_count(&self, canvas: &Canvas) -> usize {
        self.base.get_line_count(canvas)
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

    pub fn line_count(&self, canvas: &Canvas) -> usize {
        self.text_sheet[self.cursor].get_line_count(canvas)
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

#[cfg(test)]
mod test {

    use super::*;
    use crate::engine::{
        consts::DEFAULT_CANVAS,
        ui::{BorderSprite, Padding},
    };
    use term::term_size;

    #[test]
    fn basetext_height_match() {
        let mut canvas = DEFAULT_CANVAS;
        if let Some(s) = term_size() {
            canvas.height = s.1 as usize;
            canvas.width = s.0 as usize;
        }
        let text = String::from("1111\n2222\n3333\n4444\n5555\n6666\n7777");

        // Senerio One //
        let target_height: usize = 40;

        let mut tb: TextBase = TextBase::new(
            text.clone(),
            Justify::Left,
            Align::Center,
            None,
            Some(Measure::Cell(target_height as u32)),
            Some(Border::from(
                BorderSprite::String("#@".to_string()),
                Padding::square(2),
            )),
            None,
            None,
        );
        let mut lines: Vec<String> = Vec::new();
        for each in tb.as_str(&canvas).split("\n") {
            lines.push(each.to_string())
        }
        assert_eq!(lines.len(), target_height);
        println!("{}", tb.as_str(&canvas));

        // Senario Two //
        let target_height: usize = canvas.height / 2;

        let mut tb: TextBase = TextBase::new(
            text.clone(),
            Justify::Left,
            Align::Center,
            None,
            Some(Measure::Percent(50)),
            Some(Border::from(
                BorderSprite::String("#@".to_string()),
                Padding::square(2),
            )),
            None,
            None,
        );
        let (l, r) = tb.get_justify_gaps(1, 7);
        println!("lpad: {}, rpad: {}", l, r);
        println!("tb.width() = {} (expected: 7)", tb.width(&canvas));
        let mut lines: Vec<String> = Vec::new();
        for each in tb.as_str(&canvas).split("\n") {
            lines.push(each.to_string());
        }
        println!("{}", tb.as_str(&canvas));
        assert_eq!(lines.len(), target_height);
    }

    fn basetext_width_match() {}

    #[test]
    fn basetext_string_width() {
        let text = String::from("Like");
        assert_eq!(TextBase::string_width(&text), text.len());
        let text = String::from("\x1b[0mLike");
        assert_eq!(TextBase::string_width(&text), text.len() - 4);
        let text = String::from("\x1b[0mLike\x1b[0m");
        assert_eq!(TextBase::string_width(&text), text.len() - 8);
        let text = String::from("hello there stuff like this\x1b[0mLike what and break up\x1bm");
        assert_eq!(TextBase::string_width(&text), text.len() - 6);
        let text = String::from("\x1b[0mLike\x1bm");
        assert_eq!(TextBase::string_width(&text), text.len() - 6);
        let text = String::from("\x1b[0mMedium\x1b[0m");
        assert_eq!(TextBase::string_width(&text), text.len() - 8);
    }
}
