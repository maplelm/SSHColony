use std::fmt::Display;

pub struct Padding {
    pub top: u32,
    pub bottom: u32,
    pub right: u32,
    pub left: u32,
}

impl Padding {
    pub fn square(s: u32) -> Self {
        Self {
            top: s,
            bottom: s,
            right: s,
            left: s,
        }
    }

    pub fn rectangle(w: u32, l: u32) -> Self {
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

pub struct Border<T: Display> {
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T,
    pub padding: Padding,
}

impl Default for Border<char> {
    fn default() -> Self {
        Self {
            top: '#',
            bottom: '#',
            left: '#',
            right: '#',
            padding: Padding::square(1),
        }
    }
}
