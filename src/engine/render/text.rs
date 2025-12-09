use super::drawable::{Line, LineSlice};
use crate::engine::{
    render::{Canvas, sprite::Position},
    types::Position3D,
    ui::style::{Align, Alignment, Justify, Style},
};
use my_term::{
    Text,
    color::{Background, Foreground},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub struct TextboxSlice<'a> {
    pub lines: Vec<LineSlice<'a>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Textbox {
    Static(Static),
    Dynamic(Dynamic),
}

impl Textbox {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Base {
    lines: Vec<Line>,
    style: Style,
    cache: Option<Vec<Line>>,
}

impl Base {
    pub fn new(lines: Vec<Line>, style: Style, can: &Canvas) -> Self {
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
        let mut cache = vec![];
        let (t_align, b_align) = self.v_alignments(can);
        let max_str_len = self.max_text_len(can);
        // Build Top Border
        if let Some(b) = self.style.border.as_ref() {
            let mut text: String = String::new();
            if b.has_left_border() {
                text.push(b.top_l().unwrap_or(' '));
            }
            if b.has_top_border() {
                for col in 0..self.width(can) - 2 {
                    text.push(b.top(col).unwrap_or(' '));
                }
            }
            if b.has_right_border() {
                text.push(b.top_r().unwrap_or(' '));
            }
            cache.push(Line::new(vec![Text::new(
                text,
                self.style.fg(),
                self.style.bg(),
            )]));

            // Build Top Padding
            let width = self.width(can)
                - (if b.has_left_border() { 1 } else { 0 }
                    + if b.has_right_border() { 1 } else { 0 });
            for p_row in 0..b.top_pad() + t_align {
                let mut text: String = String::new();
                if b.has_left_border() {
                    text.push(b.l(row).unwrap_or(' '));
                }
                for _ in 0..width {
                    text.push(' ');
                }
                if b.has_right_border() {
                    text.push(b.r(row).unwrap_or(' '));
                }
                cache.push(Line::new(vec![Text::new(
                    text,
                    self.style.fg(),
                    self.style.bg(),
                )]));
                row += 1;
            }
        } else {
            // Build Top Alignment
            if t_align > 0 {
                let mut text = String::new();
                for _ in 0..self.width(can) {
                    text.push(' ')
                }
                for _ in 0..t_align {
                    cache.push(Line::new(vec![Text::new(
                        text.clone(),
                        self.style.fg(),
                        self.style.bg(),
                    )]));
                }
            }
        }

        // Build Lines
        for (line_row, line) in self.lines.iter().enumerate() {
            let mut text = String::new();
            let (l_align, r_align) = self.h_alignments(line_row, can);
            if let Some(b) = self.style.border.as_ref() {
                if b.has_left_border() {
                    text.push(b.l(row).unwrap_or(' '));
                }
                for _ in 0..b.l_pad() {
                    text.push(' ');
                }
            }
            for _ in 0..l_align {
                text.push(' ');
            }
            text.push_str(&format!("{}", line.slice(0, max_str_len)));
            for _ in 0..r_align {
                text.push(' ');
            }
            if let Some(b) = self.style.border.as_ref() {
                for _ in 0..b.r_pad() {
                    text.push(' ');
                }
                if b.has_right_border() {
                    text.push(b.r(row).unwrap_or(' '));
                }
            }
            cache.push(Line::new(vec![Text::new(
                text,
                self.style.fg(),
                self.style.bg(),
            )]));
            row += 1;
        }

        // Build Bot Padding
        if let Some(b) = self.style.border.as_ref() {
            let width = self.width(can)
                - (if b.has_left_border() { 1 } else { 0 }
                    + if b.has_right_border() { 1 } else { 0 });
            for _ in 0..b_align {
                let mut text = String::new();
                if b.has_left_border() {
                    text.push(b.l(row).unwrap_or(' '));
                }
                for _ in 0..width {
                    text.push(' ');
                }
                if b.has_right_border() {
                    text.push(b.r(row).unwrap_or(' '));
                }
                row += 1;
                cache.push(Line::new(vec![Text::new(
                    text,
                    self.style.fg(),
                    self.style.bg(),
                )]));
            }

            for _ in 0..b.bot_pad() {
                let mut text = String::new();
                if b.has_left_border() {
                    text.push(b.l(row).unwrap_or(' '));
                }
                for _ in 0..width {
                    text.push(' ');
                }
                if b.has_right_border() {
                    text.push(b.r(row).unwrap_or(' '));
                }
                row += 1;
                cache.push(Line::new(vec![Text::new(
                    text,
                    self.style.fg(),
                    self.style.bg(),
                )]));
            }
        } else {
            if b_align > 0 {
                let mut text = String::new();
                for _ in 0..self.width(can) {
                    text.push(' ');
                }
                for _ in 0..b_align {
                    cache.push(Line::new(vec![Text::new(
                        text.clone(),
                        self.style.fg(),
                        self.style.bg(),
                    )]));
                }
            }
        }

        // Build Bot Border
        if let Some(b) = self.style.border.as_ref() {
            let width = self.width(can)
                - (if b.has_left_border() { 1 } else { 0 }
                    + if b.has_right_border() { 1 } else { 0 });
            let mut text = String::new();
            if b.has_left_border() {
                text.push(b.bot_l().unwrap_or(' '));
            }
            for i in 0..width {
                text.push(b.bot(i).unwrap_or(' '));
            }
            if b.has_right_border() {
                text.push(b.bot_r().unwrap_or(' '));
            }
            cache.push(Line::new(vec![Text::new(
                text,
                self.style.fg(),
                self.style.bg(),
            )]));
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
