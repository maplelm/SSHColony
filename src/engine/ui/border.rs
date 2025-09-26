use std::clone::Clone;
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::str;

#[derive(Clone)]
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
}

#[derive(Clone)]
struct Side {
    sprite: BorderSprite,
    cursor: usize,
}

impl Side {
    pub fn new(s: BorderSprite) -> Self {
        Self {
            sprite: s,
            cursor: 0,
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Border {
    top: Side,
    bottom: Side,
    left: Side,
    right: Side,
    padding: Padding,
}

impl Border {
    pub fn new(s: BorderSprite, p: Padding) -> Self {
        Self {
            top: Side::new(s.clone()),
            bottom: Side::new(s.clone()),
            left: Side::new(s.clone()),
            right: Side::new(s.clone()),
            padding: p,
        }
    }

    pub fn new_detailed(
        t: BorderSprite,
        b: BorderSprite,
        l: BorderSprite,
        r: BorderSprite,
        p: Padding,
    ) -> Self {
        Self {
            top: Side::new(t),
            bottom: Side::new(b),
            left: Side::new(l),
            right: Side::new(r),
            padding: p,
        }
    }

    pub fn width(&self) -> usize {
        if !self.left.sprite.is_none() || !self.right.sprite.is_none() {
            if !self.left.sprite.is_none() && !self.right.sprite.is_none() {
                return 2;
            }
            return 1;
        }
        return 0;
    }

    pub fn height(&self) -> usize {
        if !self.top.sprite.is_none() || !self.bottom.sprite.is_none() {
            if !self.top.sprite.is_none() && !self.bottom.sprite.is_none() {
                return 2;
            }
            return 1;
        }
        return 0;
    }

    pub fn padding_top(&self) -> usize {
        self.padding.top
    }

    pub fn padding_bottom(&self) -> usize {
        self.padding.bottom
    }

    pub fn padding_left(&self) -> usize {
        self.padding.left
    }

    pub fn padding_right(&self) -> usize {
        self.padding.right
    }

    pub fn reset(&mut self) {
        self.top.cursor = 0;
        self.bottom.cursor = 0;
        self.left.cursor = 0;
        self.right.cursor = 0;
    }

    pub fn reset_top(&mut self) {
        self.top.cursor = 0;
    }

    pub fn reset_bottom(&mut self) {
        self.bottom.cursor = 0;
    }

    pub fn reset_left(&mut self) {
        self.left.cursor = 0;
    }

    pub fn reset_right(&mut self) {
        self.right.cursor = 0;
    }

    pub fn next_top(&mut self) -> char {
        match &self.top.sprite {
            BorderSprite::None => '\0',
            BorderSprite::Char(c) => *c,
            BorderSprite::String(s) => {
                let mut output: char = match s.chars().nth(self.top.cursor) {
                    Some(c) => c,
                    None => '\0', // Log that There is a problem
                };
                if self.top.cursor == s.len() - 1 {
                    self.top.cursor = 0;
                } else {
                    self.top.cursor += 1;
                }
                return output;
            }
        }
    }

    pub fn next_bottom(&mut self) -> char {
        match &self.bottom.sprite {
            BorderSprite::None => '\0',
            BorderSprite::Char(c) => *c,
            BorderSprite::String(s) => {
                let output: char = match s.chars().nth(self.bottom.cursor) {
                    Some(c) => c,
                    None => '\0',
                };
                if self.bottom.cursor == s.len() - 1 {
                    self.bottom.cursor = 0;
                } else {
                    self.bottom.cursor += 1;
                }
                return output;
            }
        }
    }

    pub fn next_left(&mut self) -> char {
        match &self.left.sprite {
            BorderSprite::None => '\0',
            BorderSprite::Char(c) => *c,
            BorderSprite::String(s) => {
                let output: char = match s.chars().nth(self.left.cursor) {
                    Some(c) => c,
                    None => '\0',
                };
                if self.left.cursor == s.len() - 1 {
                    self.left.cursor = 0;
                } else {
                    self.left.cursor += 1;
                }
                return output;
            }
        }
    }

    pub fn next_right(&mut self) -> char {
        match &self.right.sprite {
            BorderSprite::None => '\0',
            BorderSprite::Char(c) => *c,
            BorderSprite::String(s) => {
                let output: char = match s.chars().nth(self.right.cursor) {
                    Some(c) => c,
                    None => '\0',
                };
                if self.right.cursor == s.len() - 1 {
                    self.right.cursor = 0;
                } else {
                    self.right.cursor += 1;
                }
                return output;
            }
        }
    }
}

impl Default for Border {
    fn default() -> Self {
        Self {
            top: Side::new(BorderSprite::Char('#')),
            bottom: Side::new(BorderSprite::Char('#')),
            left: Side::new(BorderSprite::Char('#')),
            right: Side::new(BorderSprite::Char('#')),
            padding: Padding::square(1),
        }
    }
}
