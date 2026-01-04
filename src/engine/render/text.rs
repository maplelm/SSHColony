use crate::engine::render::drawable::Text;
use crate::engine::{
    render::{Canvas, Char, drawable::TextSlice, sprite::Position},
    types::Position3D,
    ui::style::{Align, Alignment, Justify, Style},
};
use my_term::color::{Background, Foreground};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Write},
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct TextboxSlice<'a> {
    pub lines: Vec<TextSlice<'a>>,
}

impl<'a> std::fmt::Display for TextboxSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, l) in self.lines.iter().enumerate() {
            if i < self.lines.len() - 1 {
                if let Err(e) = write!(f, "{}\x1b[1B\x1b[{}D", l, l.len()) {
                    return std::fmt::Result::Err(e);
                }
            } else {
                if let Err(e) = write!(f, "{l}") {
                    return std::fmt::Result::Err(e);
                }
            }
        }
        std::fmt::Result::Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Textbox {
    Static(Static),
    Dynamic(Dynamic),
}

impl Textbox {
    pub fn new_static(pos: Position3D<i32>, lines: Vec<Text>, style: Style, can: &Canvas) -> Self {
        Self::Static(Static {
            pos,
            base: Base::new(lines, style, can),
        })
    }

    pub fn new_dynamic(pos: Position3D<i32>, frames: Vec<Base>, tick_rate: Duration) -> Self {
        Self::Dynamic(Dynamic {
            pos,
            frames,
            cursor: 0,
            tick_rate,
            last_tick: None,
        })
    }

    pub fn width(&self, can: &Canvas) -> usize {
        match self {
            Self::Static(s) => s.base.width(can),
            Self::Dynamic(d) => d.frames[d.cursor].width(can),
        }
    }

    pub fn height(&self, can: &Canvas) -> usize {
        match self {
            Self::Static(s) => s.base.height(can),
            Self::Dynamic(d) => d.frames[d.cursor].height(can),
        }
    }

    pub fn pos(&self) -> Position3D<i32> {
        match self {
            Self::Static(s) => s.pos.clone(),
            Self::Dynamic(d) => d.pos.clone(),
        }
    }

    pub fn move_pos(&mut self, pos: Position3D<i32>) {
        match self {
            Self::Static(s) => s.pos += pos,
            Self::Dynamic(d) => d.pos += pos,
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Self::Static(_) => true,
            _ => false,
        }
    }

    pub fn insert(&mut self, row: usize, col: usize, chars: &[Char], canvas: &Canvas) {
        match self {
            Self::Static(s) => {
                if row == s.base.lines.len() {
                    s.base.lines.push(Text::new());
                }
                for i in 0..chars.len() {
                    s.base.lines[row].insert(col + i, chars[i]);
                }
            }
            Self::Dynamic(d) => {
                let lines = &mut d.frames[d.cursor].lines;
                if row == lines.len() {
                    lines.push(Text::new());
                }
                for i in 0..chars.len() {
                    lines[row].insert(col + i, chars[i]);
                }
            }
        }
    }

    pub fn push(&mut self, text: Text, canvas: &Canvas) {
        match self {
            Self::Static(s) => {
                s.base.lines.push(text);
                s.base.build_cache(canvas);
            }
            Self::Dynamic(d) => {
                d.frames[d.cursor].lines.push(text);
                d.frames[d.cursor].build_cache(canvas);
            }
        }
    }

    pub fn slice(
        &self,
        left_offset: usize,
        right_offset: usize,
        top_offset: usize,
        bot_offset: usize,
    ) -> TextboxSlice {
        let mut v = vec![];
        match self {
            Self::Static(s) => {
                let bot_offset = s.base.cache.as_ref().unwrap().len() - bot_offset;
                for (i, l) in s.base.cache.as_ref().unwrap().iter().enumerate() {
                    if i < top_offset {
                        continue;
                    }
                    if i >= bot_offset {
                        break;
                    }
                    let right_offset = l.len() - right_offset;
                    v.push(l.slice(left_offset, right_offset));
                }
            }
            Self::Dynamic(d) => {
                let lines = d.frames[d.cursor].cache.as_ref().unwrap();
                let bot_offset = lines.len() - bot_offset;
                for (i, l) in lines.iter().enumerate() {
                    if i < top_offset {
                        continue;
                    }
                    if i > bot_offset {
                        break;
                    }
                    let right_offset = l.len() - right_offset;
                    v.push(l.slice(left_offset, right_offset));
                }
            }
        }
        TextboxSlice { lines: v }
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Dynamic(_) => true,
            _ => false,
        }
    }

    pub fn update(&mut self) -> bool {
        match self {
            Self::Dynamic(d) => {
                if d.last_tick.is_some() && d.last_tick.unwrap().elapsed() > d.tick_rate {
                    d.cursor = (d.cursor + 1) % d.frames.len();
                    d.last_tick = Some(Instant::now());
                    return true;
                } else if d.last_tick.is_none() {
                    d.last_tick = Some(Instant::now());
                }
                return false;
            }
            Self::Static(_) => false,
        }
    }
}

impl std::fmt::Display for Textbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let print = |f: &mut std::fmt::Formatter<'_>, lines: &[Text]| -> std::fmt::Result {
            for (i, l) in lines.iter().enumerate() {
                if i < lines.len() - 1 {
                    if let Err(e) = write!(f, "{}[1B[{}D\n", l, l.len()) {
                        return std::fmt::Result::Err(e);
                    }
                } else {
                    if let Err(e) = write!(f, "{l}") {
                        return std::fmt::Result::Err(e);
                    }
                }
            }
            std::fmt::Result::Ok(())
        };
        match self {
            Self::Static(s) => match &s.base.cache {
                Some(cache) => print(f, cache),
                None => std::fmt::Result::Ok(()),
            },
            Self::Dynamic(d) => match d.frames[d.cursor].cache.as_ref() {
                Some(cache) => print(f, cache),
                None => std::fmt::Result::Ok(()),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Base {
    lines: Vec<Text>,
    style: Style,
    cache: Option<Vec<Text>>,
}

impl Base {
    pub fn new(lines: Vec<Text>, style: Style, can: &Canvas) -> Self {
        let mut output = Self {
            lines,
            style,
            cache: None,
        };
        output.build_cache(can);
        output
    }

    pub fn width(&self, can: &Canvas) -> usize {
        if let Some(measure) = self.style.width() {
            measure.get(can.width)
        } else {
            let mut w = 0;
            for line in self.lines.iter() {
                if w < line.len() {
                    w = line.len()
                }
            }
            if let Some(b) = self.style.border.as_ref() {
                w + b.width()
            } else {
                w
            }
        }
    }

    pub fn height(&self, can: &Canvas) -> usize {
        if let Some(measure) = self.style.height() {
            measure.get(can.height)
        } else {
            let mut h = 0;
            for line in self.lines.iter() {
                if h < line.len() {
                    h = line.len()
                }
            }
            if let Some(b) = self.style.border.as_ref() {
                h + b.width()
            } else {
                h
            }
        }
    }

    pub fn slice(&self, t: usize, b: usize, l: usize, r: usize) -> TextboxSlice {
        let mut slice = vec![];
        if self.cache.is_none() {
            return TextboxSlice { lines: vec![] };
        }
        for (i, line) in self.cache.as_ref().unwrap().iter().enumerate() {
            if i < t {
                continue;
            } else if i > b {
                break;
            }
            let r = line.len() - r;
            slice.push(line.slice(l, r));
        }
        TextboxSlice { lines: slice }
    }

    pub fn build_cache(&mut self, can: &Canvas) {
        let mut row = 0;
        let mut cache: Vec<Text> = Vec::with_capacity(self.height(can));
        let (t_align, b_align) = self.v_alignments(can);
        let max_str_len = self.max_text_len(can);
        let fg = self.style.fg();
        let bg = self.style.bg();

        // Build Top Border
        if let Some(b) = self.style.border.as_ref() {
            let mut text: Text = Text::new();
            if b.has_left_border() {
                text.push(Char::new(b.top_l().unwrap_or(' '), fg, bg));
            }
            if b.has_top_border() {
                for col in 0..self.width(can) - 2 {
                    text.push(Char::new(b.top(col).unwrap_or(' '), fg, bg));
                }
            }
            if b.has_right_border() {
                text.push(Char::new(b.top_r().unwrap_or(' '), fg, bg));
            }
            cache.push(text);

            // Build Top Padding
            let width = self.width(can)
                - (if b.has_left_border() { 1 } else { 0 }
                    + if b.has_right_border() { 1 } else { 0 });
            for p_row in 0..b.top_pad() + t_align {
                let mut text: Text = Text::new();
                if b.has_left_border() {
                    text.push(Char::new(b.l(row).unwrap_or(' '), fg, bg));
                }
                for _ in 0..width {
                    text.push(Char::new(' ', fg, bg));
                }
                if b.has_right_border() {
                    text.push(Char::new(b.r(row).unwrap_or(' '), fg, bg));
                }
                cache.push(text);
                row += 1;
            }
        } else {
            // Build Top Alignment
            if t_align > 0 {
                let mut text = Text::new();
                for _ in 0..self.width(can) {
                    text.push(Char::new(' ', fg, bg));
                }
                for _ in 0..t_align {
                    cache.push(text.clone());
                }
            }
        }

        // Build Lines
        for (line_row, line) in self.lines.iter().enumerate() {
            let mut text = Text::new();
            let (l_align, r_align) = self.h_alignments(line_row, can);
            if let Some(b) = self.style.border.as_ref() {
                if b.has_left_border() {
                    text.push(Char::new(b.l(row).unwrap_or(' '), fg, bg));
                }
                for _ in 0..b.l_pad() {
                    text.push(Char::new(' ', fg, bg));
                }
            }
            for _ in 0..l_align {
                text.push(Char::new(' ', fg, bg));
            }
            text.push_textslice(&line.slice(0, max_str_len));
            for _ in 0..r_align {
                text.push(Char::new(' ', fg, bg));
            }
            if let Some(b) = self.style.border.as_ref() {
                for _ in 0..b.r_pad() {
                    text.push(Char::new(' ', fg, bg));
                }
                if b.has_right_border() {
                    text.push(Char::new(b.r(row).unwrap_or(' '), fg, bg));
                }
            }
            cache.push(text);
            row += 1;
        }

        // Build Bot Padding
        if let Some(b) = self.style.border.as_ref() {
            let width = self.width(can)
                - (if b.has_left_border() { 1 } else { 0 }
                    + if b.has_right_border() { 1 } else { 0 });
            for _ in 0..b_align {
                let mut text = Text::new();
                if b.has_left_border() {
                    text.push(Char::new(b.l(row).unwrap_or(' '), fg, bg));
                }
                for _ in 0..width {
                    text.push(Char::new(' ', fg, bg));
                }
                if b.has_right_border() {
                    text.push(Char::new(b.r(row).unwrap_or(' '), fg, bg));
                }
                row += 1;
                cache.push(text);
            }

            for _ in 0..b.bot_pad() {
                let mut text = Text::new();
                if b.has_left_border() {
                    text.push(Char::new(b.l(row).unwrap_or(' '), fg, bg));
                }
                for _ in 0..width {
                    text.push(Char::new(' ', fg, bg));
                }
                if b.has_right_border() {
                    text.push(Char::new(b.r(row).unwrap_or(' '), fg, bg));
                }
                row += 1;
                cache.push(text);
            }
        } else {
            if b_align > 0 {
                let mut text = Text::new();
                for _ in 0..self.width(can) {
                    text.push(Char::new(' ', fg, bg));
                }
                for _ in 0..b_align {
                    cache.push(text.clone());
                }
            }
        }

        // Build Bot Border
        if let Some(b) = self.style.border.as_ref() {
            let width = self.width(can)
                - (if b.has_left_border() { 1 } else { 0 }
                    + if b.has_right_border() { 1 } else { 0 });
            let mut text = Text::new();
            if b.has_left_border() {
                text.push(Char::new(b.bot_l().unwrap_or(' '), fg, bg));
            }
            for i in 0..width {
                text.push(Char::new(b.bot(i).unwrap_or(' '), fg, bg));
            }
            if b.has_right_border() {
                text.push(Char::new(b.bot_r().unwrap_or(' '), fg, bg));
            }
            cache.push(text);
        }
        self.cache = Some(cache);
    }

    pub fn v_alignments(&self, can: &Canvas) -> (usize, usize) {
        let mut total = self.height(can);
        if let Some(b) = self.style.border.as_ref() {
            if b.has_top_border() {
                total -= 1;
            }
            total -= b.top_pad() + b.bot_pad();
            if b.has_bot_border() {
                total -= 1;
            }
        }
        total -= self.lines.len();

        match self.style.alignment.align {
            Align::Top => (0, total),
            Align::Bottom => (total, 0),
            Align::Center => {
                if total % 2 == 0 {
                    (total / 2, total / 2)
                } else {
                    ((total / 2) + 1, total / 2)
                }
            }
        }
    }

    pub fn h_alignments(&self, row: usize, can: &Canvas) -> (usize, usize) {
        let mut total = self.width(can);
        if let Some(b) = self.style.border.as_ref() {
            if b.has_left_border() {
                total -= 1;
            }
            total -= b.l_pad() + b.r_pad();
            if b.has_right_border() {
                total -= 1;
            }
        }
        total -= self.lines[row].len();
        match self.style.justify() {
            Justify::Left => (0, total),
            Justify::Right => (total, 0),
            Justify::Center => {
                if total % 2 == 0 {
                    (total / 2, total / 2)
                } else {
                    ((total / 2) + 1, total / 2)
                }
            }
        }
    }

    pub fn max_text_len(&self, can: &Canvas) -> usize {
        let mut max = self.width(can);
        if let Some(b) = self.style.border.as_ref() {
            if b.has_left_border() {
                max -= 1;
            }
            max -= b.l_pad() + b.r_pad();
            if b.has_right_border() {
                max -= 1;
            }
        }
        max
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Static {
    pub pos: Position3D<i32>,
    pub base: Base,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dynamic {
    pub pos: Position3D<i32>,
    pub frames: Vec<Base>,
    pub cursor: usize,
    pub tick_rate: Duration,
    #[serde(skip)]
    pub last_tick: Option<Instant>,
}
