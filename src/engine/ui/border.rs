use std::clone::Clone;
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::str;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Border {
    top: BorderSprite,
    bottom: BorderSprite,
    left: BorderSprite,
    right: BorderSprite,
    padding: Padding,
}

impl Border {
    pub fn new() -> Self {
        Self { 
            top: BorderSprite::None,
            bottom: BorderSprite::None,
            left: BorderSprite::None, 
            right: BorderSprite::None,
            padding: Padding { top: 0, bottom: 0, right: 0, left: 0 }
        }
    }

    pub fn is_top_none(&self) ->bool {
        match self.top {
            BorderSprite::None => true,
            _ => false
        }
    }

    pub fn is_bottom_none(&self) ->bool {
        match self.bottom {
            BorderSprite::None => true,
            _ => false
        }
    }

    pub fn is_left_none(&self) ->bool {
        match self.left {
            BorderSprite::None => true,
            _ => false
        }
    }

    pub fn is_right_none(&self) ->bool {
        match self.right {
            BorderSprite::None => true,
            _ => false
        }
    }

    pub fn from_str(s: &str) -> Self {
        Self {
            top: BorderSprite::String(s.to_string()),
            bottom: BorderSprite::String(s.to_string()),
            left: BorderSprite::String(s.to_string()),
            right: BorderSprite::String(s.to_string()),
            padding: Padding::square(0)
        }
    }

    pub fn from_string(s: &String) -> Self {
        Self {
            top: BorderSprite::String(s.clone()),
            bottom: BorderSprite::String(s.clone()),
            left: BorderSprite::String(s.clone()),
            right: BorderSprite::String(s.clone()),
            padding: Padding::square(0)
        }
    } 


    pub fn top(mut self, sprite: BorderSprite) -> Self {
        self.top = sprite;
        self
    }

    pub fn bottom(mut self, sprite: BorderSprite) -> Self {
        self.bottom = sprite;
        self
    }

    pub fn left(mut self, sprite: BorderSprite) -> Self {
        self.left = sprite;
        self
    }

    pub fn right(mut self, sprite: BorderSprite) -> Self {
        self.right = sprite;
        self
    }

    pub fn pad_top(mut self, val: usize) -> Self {
        self.padding.top = val;
        self
    }

    pub fn pad_bottom(mut self, val: usize) -> Self {
        self.padding.bottom = val;
        self
    }

    pub fn pad_left(mut self, val: usize) -> Self {
        self.padding.left = val;
        self
    }

    pub fn pad(mut self, pad: Padding) -> Self {
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
            bottom: s.clone(),
            left: s.clone(),
            right: s.clone(),
            padding: p,
        }
    }


    pub fn width(&self) -> usize {
        if !self.left.is_none() || !self.right.is_none() {
            if !self.left.is_none() && !self.right.is_none() {
                return 2;
            }
            return 1;
        }
        return 0;
    }

    pub fn height(&self) -> usize {
        if !self.top.is_none() || !self.bottom.is_none() {
            if !self.top.is_none() && !self.bottom.is_none() {
                return 2;
            }
            return 1;
        }
        return 0;
    }

    pub fn get_pad_top(&self) -> usize {
        self.padding.top
    }

    pub fn get_pad_bottom(&self) -> usize {
        self.padding.bottom
    }

    pub fn get_pad_left(&self) -> usize {
        self.padding.left
    }

    pub fn get_pad_right(&self) -> usize {
        self.padding.right
    }


    pub fn get_top(&self, iter: usize) -> Option<char> {
        match &self.top {
            BorderSprite::None => None,
            BorderSprite::Char(c) => Some(*c),
            BorderSprite::String(s) => s.chars().nth(iter % s.len()),
        }
    }

    pub fn get_bottom(&self, iter: usize) -> Option<char> {
        match &self.bottom {
            BorderSprite::None => None,
            BorderSprite::Char(c) => Some(*c),
            BorderSprite::String(s) => s.chars().nth(iter % s.len()),
        }
    }

    pub fn get_left(&self, iter: usize) -> Option<char> {
        match &self.left {
            BorderSprite::None => None,
            BorderSprite::Char(c) => Some(*c),
            BorderSprite::String(s) => s.chars().nth(iter % s.len()),
        }
    }

    pub fn get_right(&self, iter: usize) -> Option<char> {
        match &self.right {
            BorderSprite::None => None,
            BorderSprite::Char(c) => Some(*c),
            BorderSprite::String(s) => s.chars().nth(iter % s.len())
        }
    }
}

impl Default for Border {
    fn default() -> Self {
        Self {
            top: BorderSprite::Char('#'),
            bottom: BorderSprite::Char('#'),
            left: BorderSprite::Char('#'),
            right: BorderSprite::Char('#'),
            padding: Padding::square(1),
        }
    }
}
