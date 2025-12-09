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
use crate::engine::ui::style::{Align, Justify, LIGHT_BLOCK, Style};
use crate::engine::ui::{Border, style::Measure};
use my_term::{
    Text,
    TextSlice,
    color::{Background, Foreground},
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Write;
use std::time::{Duration, Instant};

////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Text Line ///////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct LineSlice<'a> {
    len: usize,
    sections: Vec<TextSlice<'a>>,
}

impl<'a> Display for LineSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for sec in self.sections.iter() {
            output.push_str(&format!("{sec}",));
        }

        write!(f, "{output}")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Line {
    len: usize,
    sections: Vec<Text>,
}

impl Line {
    pub fn from(t: impl Into<String>, fg: &Foreground, bg: &Background) -> Self {
        let mut lines = Vec::new();
        let mut len: usize = 0;
        for (i, line) in t.into().split("\n").enumerate() {
            len += line.len();
            lines.push(Text::new(line, fg, bg));
        }

        Self {
            len,
            sections: lines,
        }
    }
    pub fn new(sections: Vec<Text>) -> Self {
        let mut len = 0;
        for secs in sections.iter() {
            len += secs.len();
        }
        Self { len, sections }
    }

    pub fn set(&mut self, s: Vec<Text>) {
        let mut len = 0;
        for secs in self.sections.iter() {
            len += secs.len();
        }
        self.len = len;
        self.sections = s;
    }

    pub fn set_str(&mut self, s: impl Into<String>) {
        let s: String = s.into();
        let len = s.len();
        self.len = len;
        self.sections = vec![Text::new(s, &Foreground::none(), &Background::none())];
    }

    pub fn push_section(&mut self, t: &Text) {
        self.sections.push((*t).clone());
    }

    pub fn push_str(&mut self, s: impl Into<String>) {
        self.sections.push(Text::from(s));
    }

    pub fn truncate(&mut self, s: usize, e: usize) {
        if s >= e || s >= self.len() || e > self.len() {
            return;
        }


        let mut start_cursor: usize = 0;
        let mut start_block: usize = 0;
        let mut start_inner_block: usize = 0;
        for sec in self.sections.iter() {
            if s > sec.len() + start_cursor {
                start_cursor += sec.len();
                start_block += 1;
            } else {
                start_inner_block = s - start_cursor;
                break;
            }
        }

        let mut end_cursor: usize = 0;
        let mut end_block: usize = 0;
        let mut end_inner_block: usize = 0;
        let e = self.len() - e;
        for sec in self.sections.iter().rev() {
            if e > sec.len() + end_cursor {
                end_block += 1;
                end_cursor += sec.len();
            } else {
                end_inner_block = e - end_cursor;
                break;
            }
        }

        self.sections.drain(..start_block);
        let slice = self.sections[0].slice(0, start_inner_block);
        self.sections[0] = slice.into();

        self.sections.truncate(self.sections.len() - end_block);
        let last_ele = self.sections.len() -1;
        let slice = self.sections[last_ele].slice(self.sections[last_ele].len() - end_inner_block, self.sections[last_ele].len());
        self.sections[last_ele] = slice.into();

    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> LineSlice {
        let mut slice = vec![];
        for line in self.sections.iter() {
            slice.push(line.as_slice())
        }
        LineSlice { len: self.len(), sections: slice}
    }

    pub fn slice(&self, min: usize, max: usize) -> LineSlice {
        let max = if max > self.len() {self.len()} else {max};
        let mut cursor= 0;
        let mut start_block = 0;
        let mut start_block_offset = 0;
        let mut end_block = 0;
        let mut end_block_offset = 0;

        for sec in self.sections.iter() {
            if min > sec.len() + cursor {
                start_block += 1;
                cursor += sec.len();
            } else {
                start_block_offset = min - cursor;
                break;
            }
        }

        let invert_max = self.len() - max;
        cursor = 0;
        for sec in self.sections.iter().rev() {
            if invert_max > sec.len() + cursor {
                end_block += 1;
                cursor += sec.len();
            } else {
                end_block_offset = invert_max - cursor;
                break;
            }
        }

        let mut slice_len = 0;
        let text_slice = &self.sections[start_block..self.sections.len() - end_block];
        let mut new_slice = vec![];
        for (index, text) in self.sections[start_block..self.sections.len()-end_block].iter().enumerate() {
            if text_slice.len() == 1 {
               new_slice.push(text.slice(start_block_offset, text.len() - end_block_offset));
           } else if index == 0 {
            new_slice.push(text.slice(start_block_offset, text.len()));
           } else if index == text_slice.len() - 1 {
            new_slice.push(text.slice(0, text.len() - end_block_offset));
           } else {
            new_slice.push(text.as_slice());
           }
        }
        for sec in new_slice.iter() {
            slice_len += sec.len();
        }
        LineSlice {
            len: slice_len,
            sections: new_slice,
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for sec in self.sections.iter() {
            string.push_str(&format!("{sec}"));
        }
        write!(f, "{}", string)
    }
}