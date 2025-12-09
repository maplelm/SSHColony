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

use std::clone::Clone;
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::str;

use crate::engine::ui::style::*;

#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum BorderSprite {
    Char(char),
    String(String),
    None,
}

impl BorderSprite {
    pub fn is_none(&self) -> bool {
        match self {
            BorderSprite::None => true,
            _ => false,
        }
    }

    pub fn next(&self, iter: usize) -> Option<char> {
        match self {
            BorderSprite::Char(c) => Some(*c),
            BorderSprite::String(s) => s.chars().nth(iter % s.len()),
            BorderSprite::None => None,
        }
    }
}

#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct Padding {
    pub top: usize,
    pub bottom: usize,
    pub right: usize,
    pub left: usize,
}

impl Padding {
    pub fn square(s: usize) -> Self {
        Self {
            top: s,
            bottom: s,
            right: s,
            left: s,
        }
    }

    pub fn rectangle(w: usize, l: usize) -> Self {
        Self {
            top: l,
            bottom: l,
            right: w,
            left: w,
        }
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            top: 0,
            bottom: 0,
            right: 0,
            left: 0,
        }
    }
}

#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct Border {
    top: BorderSprite,
    top_left_corner: Option<char>,
    top_right_corner: Option<char>,
    bottom: BorderSprite,
    bottom_left_corner: Option<char>,
    bottom_right_corner: Option<char>,
    left: BorderSprite,
    right: BorderSprite,
    padding: Padding,
}

impl Border {
    pub fn new() -> Self {
        Self {
            top: BorderSprite::None,
            top_left_corner: None,
            top_right_corner: None,
            bottom: BorderSprite::None,
            bottom_left_corner: None,
            bottom_right_corner: None,
            left: BorderSprite::None,
            right: BorderSprite::None,
            padding: Padding {
                top: 0,
                bottom: 0,
                right: 0,
                left: 0,
            },
        }
    }

    pub fn as_heavy(pad: Padding) -> Self {
        Self {
            top: BorderSprite::Char(HEAVY_H_LINE),
            top_right_corner: Some(HEAVY_TRC),
            top_left_corner: Some(HEAVY_TLC),
            bottom: BorderSprite::Char(HEAVY_H_LINE),
            bottom_right_corner: Some(HEAVY_BRC),
            bottom_left_corner: Some(HEAVY_BLC),
            right: BorderSprite::Char(HEAVY_V_LINE),
            left: BorderSprite::Char(HEAVY_V_LINE),
            padding: pad,
        }
    }

    pub fn as_double(pad: Padding) -> Self {
        Self {
            top: BorderSprite::Char(DOUBLE_H_LINE),
            top_right_corner: Some(DOUBLE_TRC),
            top_left_corner: Some(DOUBLE_TLC),
            bottom: BorderSprite::Char(DOUBLE_H_LINE),
            bottom_right_corner: Some(DOUBLE_BRC),
            bottom_left_corner: Some(DOUBLE_BLC),
            right: BorderSprite::Char(DOUBLE_V_LINE),
            left: BorderSprite::Char(DOUBLE_V_LINE),
            padding: pad,
        }
    }

    pub fn as_block(pad: Padding) -> Self {
        Self {
            top: BorderSprite::Char(TOP_BLOCK),
            top_right_corner: Some(FULL_BLOCK),
            top_left_corner: Some(FULL_BLOCK),
            bottom: BorderSprite::Char(BOTTOM_BLOCK),
            bottom_right_corner: Some(FULL_BLOCK),
            bottom_left_corner: Some(FULL_BLOCK),
            right: BorderSprite::Char(FULL_BLOCK),
            left: BorderSprite::Char(FULL_BLOCK),
            padding: pad,
        }
    }

    pub fn as_hash(pad: Padding) -> Self {
        Self {
            top: BorderSprite::Char('#'),
            top_right_corner: Some('#'),
            top_left_corner: Some('#'),
            bottom: BorderSprite::Char('#'),
            bottom_right_corner: Some('#'),
            bottom_left_corner: Some('#'),
            right: BorderSprite::Char('#'),
            left: BorderSprite::Char('#'),
            padding: pad,
        }
    }

    pub fn is_top_none(&self) -> bool {
        match self.top {
            BorderSprite::None => true,
            _ => false,
        }
    }

    pub fn is_top_left_corner_none(&self) -> bool {
        self.top_left_corner.is_none()
    }

    pub fn is_top_right_corner_none(&self) -> bool {
        self.top_right_corner.is_none()
    }

    pub fn is_bottom_none(&self) -> bool {
        match self.bottom {
            BorderSprite::None => true,
            _ => false,
        }
    }

    pub fn is_bottom_left_corner_none(&self) -> bool {
        self.bottom_left_corner.is_none()
    }

    pub fn is_bottom_right_corner_none(&self) -> bool {
        self.bottom_right_corner.is_none()
    }

    pub fn is_left_none(&self) -> bool {
        match self.left {
            BorderSprite::None => true,
            _ => false,
        }
    }

    pub fn is_right_none(&self) -> bool {
        match self.right {
            BorderSprite::None => true,
            _ => false,
        }
    }

    pub fn from_str(s: &str) -> Self {
        Self {
            top: BorderSprite::String(s.to_string()),
            top_left_corner: None,
            top_right_corner: None,
            bottom: BorderSprite::String(s.to_string()),
            bottom_left_corner: None,
            bottom_right_corner: None,
            left: BorderSprite::String(s.to_string()),
            right: BorderSprite::String(s.to_string()),
            padding: Padding::square(0),
        }
    }

    pub fn from_string(s: &String) -> Self {
        Self {
            top: BorderSprite::String(s.clone()),
            top_left_corner: None,
            top_right_corner: None,
            bottom: BorderSprite::String(s.clone()),
            bottom_left_corner: None,
            bottom_right_corner: None,
            left: BorderSprite::String(s.clone()),
            right: BorderSprite::String(s.clone()),
            padding: Padding::square(0),
        }
    }

    pub fn set_top(mut self, sprite: BorderSprite) -> Self {
        self.top = sprite;
        self
    }

    pub fn set_top_l(mut self, sprite: Option<char> ) -> Self {
        self.top_left_corner = sprite;
        self
    }

    pub fn set_top_r(mut self, sprite: Option<char>) -> Self {
        self.top_right_corner = sprite;
        self
    }

    pub fn set_bot(mut self, sprite: BorderSprite) -> Self {
        self.bottom = sprite;
        self
    }

    pub fn set_bot_l(mut self, sprite: Option<char>) -> Self {
        self.bottom_left_corner = sprite;
        self
    }

    pub fn set_bot_r(mut self, sprite: Option<char>) -> Self {
        self.bottom_right_corner = sprite;
        self
    }

    pub fn set_l(mut self, sprite: BorderSprite) -> Self {
        self.left = sprite;
        self
    }

    pub fn set_r(mut self, sprite: BorderSprite) -> Self {
        self.right = sprite;
        self
    }

    pub fn set_top_pad(mut self, val: usize) -> Self {
        self.padding.top = val;
        self
    }

    pub fn set_bot_pad(mut self, val: usize) -> Self {
        self.padding.bottom = val;
        self
    }

    pub fn set_l_pad(mut self, val: usize) -> Self {
        self.padding.left = val;
        self
    }

    pub fn set_pad(mut self, pad: Padding) -> Self {
        self.padding = pad;
        self
    }

    pub fn pad_right(mut self, val: usize) -> Self {
        self.padding.right = val;
        self
    }

    pub fn from(s: BorderSprite, p: Padding) -> Self {
        Self {
            top: s.clone(),
            top_right_corner: None,
            top_left_corner: None,
            bottom: s.clone(),
            bottom_right_corner: None,
            bottom_left_corner: None,
            left: s.clone(),
            right: s.clone(),
            padding: p,
        }
    }

    /** Glyphs plus padding */
    pub fn width(&self) -> usize {
        let mut w = 0;
        if !self.left.is_none()|| self.top_left_corner.is_some() || self.bottom_left_corner.is_some() {
            w += 1
        }
        if !self.right.is_none() || self.top_right_corner.is_some() || self.bottom_right_corner.is_some() {
            w += 1
        }
        w + self.padding.left + self.padding.right
    }

    pub fn has_left_border(&self) -> bool {
        !self.is_bottom_left_corner_none() || !self.is_left_none() || !self.is_top_left_corner_none()
    }

    pub fn has_right_border(&self) -> bool {
        !self.is_bottom_right_corner_none() || !self.is_right_none() || !self.is_top_right_corner_none()
    }

    pub fn has_top_border(&self) -> bool {
        !self.is_top_left_corner_none() || !self.is_top_right_corner_none() || !self.is_top_none()
    }

    pub fn has_bot_border(&self) -> bool {
        !self.is_bottom_left_corner_none() || !self.is_bottom_right_corner_none() || !self.is_bottom_none()
    }

    /** Glyphs plus padding */
    pub fn height(&self) -> usize {
        let mut h = 0;
        if !self.top.is_none() || self.top_left_corner.is_some() || self.top_right_corner.is_some() {
            h += 1;
        }
        if !self.bottom.is_none() || self.bottom_left_corner.is_some() || self.bottom_right_corner.is_some() {
            h += 2;
        }
        h + self.padding.bottom + self.padding.top
    }

    pub fn top_pad(&self) -> usize {
        self.padding.top
    }

    pub fn bot_pad(&self) -> usize {
        self.padding.bottom
    }

    pub fn l_pad(&self) -> usize {
        self.padding.left
    }

    pub fn r_pad(&self) -> usize {
        self.padding.right
    }

    pub fn top(&self, iter: usize) -> Option<char> {
        match &self.top {
            BorderSprite::None => None,
            BorderSprite::Char(c) => Some(*c),
            BorderSprite::String(s) => s.chars().nth(iter % s.chars().count()),
        }
    }

    pub fn top_l(&self) -> Option<char> {
        self.top_left_corner
    }

    pub fn top_r(&self) -> Option<char> {
        self.top_right_corner
    }

    pub fn bot(&self, iter: usize) -> Option<char> {
        match &self.bottom {
            BorderSprite::None => None,
            BorderSprite::Char(c) => Some(*c),
            BorderSprite::String(s) => s.chars().nth(iter % s.chars().count()),
        }
    }

    pub fn bot_l(&self) -> Option<char> {
    self.bottom_left_corner
    }

    pub fn bot_r(&self) -> Option<char> {
        self.bottom_right_corner
    }

    pub fn l(&self, iter: usize) -> Option<char> {
        match &self.left {
            BorderSprite::None => None,
            BorderSprite::Char(c) => Some(*c),
            BorderSprite::String(s) => s.chars().nth(iter % s.chars().count()),
        }
    }

    pub fn r(&self, iter: usize) -> Option<char> {
        match &self.right {
            BorderSprite::None => None,
            BorderSprite::Char(c) => Some(*c),
            BorderSprite::String(s) => s.chars().nth(iter % s.chars().count()),
        }
    }
}

impl Default for Border {
    fn default() -> Self {
        Self {
            top: BorderSprite::Char('#'),
            top_left_corner: Some('#'),
            top_right_corner: Some('#'),
            bottom: BorderSprite::Char('#'),
            bottom_left_corner: Some('#'),
            bottom_right_corner: Some('#'),
            left: BorderSprite::Char('#'),
            right: BorderSprite::Char('#'),
            padding: Padding::square(1),
        }
    }
}
