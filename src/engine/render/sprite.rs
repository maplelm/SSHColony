use std::time::{Duration, Instant};
use std::{fmt::Display, time};

use my_term::{
    Character, Text,
    color::{Background, Foreground},
};
use serde::{Deserialize, Serialize};

use crate::engine::types::Position3D;

pub type Position = Position3D<i32>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Glyph {
    Small(Character),
    Block(Vec<Text>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Sprite {
    Static(Static),
    Dynamic(Dynamic),
}
impl Sprite {
    pub fn new_static(c: impl Into<Character>, pos: Position) -> Self {
        Self::Static(Static {
            pos,
            base: Base {
                sprite: Glyph::Small(c.into()),
            },
        })
    }

    pub fn new_static_block(b: Vec<impl Into<Text>>, pos: Position) -> Self {
        let mut block = vec![];
        for each in b {
            block.push(each.into());
        }
        Self::Static(Static {
            pos,
            base: Base {
                sprite: Glyph::Block(block),
            },
        })
    }

    pub fn new_dynamic(sheet: Vec<impl Into<Character>>, pos: Position) -> Self {
        let mut new_sheet: Vec<Base> = vec![];
        for each in sheet {
            new_sheet.push(Base {
                sprite: Glyph::Small(each.into()),
            })
        }
        Self::Dynamic(Dynamic {
            pos,
            cursor: 0,
            tick_rate: Duration::from_secs(1),
            last_tick: None,
            frames: new_sheet,
        })
    }

    pub fn new_dynamic_block(sheet: Vec<Vec<impl Into<Text>>>, pos: Position) -> Self {
        let mut new_sheet = vec![];
        for s in sheet {
            let mut frame = vec![];
            for each in s {
                frame.push(each.into());
            }
            new_sheet.push(frame);
        }
        let mut frames = vec![];
        for f in new_sheet {
            frames.push(Base {
                sprite: Glyph::Block(f),
            });
        }
        Self::Dynamic(Dynamic {
            pos,
            cursor: 0,
            tick_rate: Duration::from_secs(1),
            last_tick: None,
            frames: frames,
        })
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Dynamic(_) => true,
            _ => false,
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Self::Static(_) => true,
            _ => false,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            Self::Static(s) => match &s.base.sprite {
                Glyph::Small(_) => 1,
                Glyph::Block(b) => b.len(),
            },
            Self::Dynamic(d) => match &d.frames[d.cursor].sprite {
                Glyph::Small(_) => 1,
                Glyph::Block(b) => b.len(),
            },
        }
    }

    pub fn width(&self) -> usize {
        match self {
            Self::Static(s) => match &s.base.sprite {
                Glyph::Small(_) => 1,
                Glyph::Block(b) => {
                    let mut w = 0;
                    for e in b.iter() {
                        if w < e.len() {
                            w = e.len();
                        }
                    }
                    w
                }
            },
            Self::Dynamic(d) => match &d.frames[d.cursor].sprite {
                Glyph::Small(_) => 1,
                Glyph::Block(b) => {
                    let mut w = 0;
                    for e in b.iter() {
                        if w < e.len() {
                            w = e.len();
                        }
                    }
                    w
                }
            },
        }
    }

    pub fn update(&mut self) -> bool {
        match self {
            Self::Static(_) => false,
            Self::Dynamic(d) => {
                if d.last_tick.is_some() && d.last_tick.unwrap().elapsed() > d.tick_rate {
                    d.cursor += (d.cursor + 1) % d.frames.len();
                    d.last_tick = Some(Instant::now());
                    true
                } else if d.last_tick.is_none() {
                    d.last_tick = Some(Instant::now());
                    false
                } else {
                    false
                }
            }
        }
    }

    pub fn pos(&self) -> Position {
        match self {
            Self::Static(s) => s.pos,
            Self::Dynamic(d) => d.pos,
        }
    }

    pub fn move_pos(&mut self, pos: Position) {
        match self {
            Self::Static(s) => s.pos += pos,
            Self::Dynamic(d) => d.pos += pos,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Base {
    pub sprite: Glyph,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Static {
    pub pos: Position,
    pub base: Base,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Dynamic {
    pub pos: Position,
    pub cursor: usize,
    pub tick_rate: Duration,
    #[serde(skip)]
    pub last_tick: Option<std::time::Instant>,
    pub frames: Vec<Base>,
}
