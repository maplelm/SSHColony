use serde::Serialize;
use serde::Deserialize;
use crate::engine::render::drawable::text::TextSlice;

use super::Text;
use super::Character as Char;

#[derive(Clone, Debug)]
pub struct GlyphSlice<'a> {
    data: Vec<TextSlice<'a>>
}

impl<'a> std::fmt::Display for GlyphSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = self.data[0].len();
        let mut output = String::new();
        for (i, l) in self.data.iter().enumerate() {
            if i < self.data.len() - 1 {
                output.push_str(&format!("{l}\x1b[1B\x1b[{w}D"));
            } else {
                output.push_str(&format!("{l}"));
            }
        }
        write!(f, "{output}")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Glyph {
    Small(Char),
    Block(Vec<Text>),
}

impl Glyph {
    pub fn small(c: Char) -> Self {
        Self::Small(c)
    }

    /** Must be rectagular or panics */
    pub fn block(b: Vec<Text>) -> Self {
        let l = b[0].len();
        for each in b.iter() {
            if each.len() != l {
                panic!("Block Glyph must be rectangular");
            }
        }
        Self::Block(b)
    }

    pub fn is_block(&self) -> bool {
        match self {
            Self::Block(_) => true,
            _ => false,
        }
    }

    pub fn as_slice(&self) -> Option<GlyphSlice> {
        match self {
            Self::Small(_) => None,
            Self::Block(b) => {
                let mut v = Vec::new();
                for l in b {
                    v.push(l.as_slice());
                }
                Some(GlyphSlice { data: v })
            }
        }
    }
    pub fn slice(&self, top_offset: usize, bot_offset: usize, left_offset: usize, right_offset: usize) -> Option<GlyphSlice> {
        match self {
            Self::Small(_) => None,
            Self::Block(b) => {
                let mut v = Vec::new();
                for (i, l) in b.iter().enumerate() {
                    if i < top_offset {
                        continue;
                    } else if i > b.len()-bot_offset {
                        break;
                    }
                    v.push(l.slice(left_offset, l.len() - right_offset));
                }
                Some(GlyphSlice { data: v })
            }
        }
    }
}

impl std::fmt::Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Small(c) => write!(f, "{c}"),
            Self::Block(b) => {
                let w = b[0].len();
                let mut output = String::new();
                for (i, l) in b.iter().enumerate() {
                    if i < b.len()-1 {
                        write!(f, "{l}\x1b[1B\x1b[{w}D");
                    } else {
                        write!(f, "{l}");
                    }
                }
                std::fmt::Result::Ok(())
            }
        }
    }
}