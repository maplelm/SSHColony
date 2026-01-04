use std::time::{Duration, Instant};
use std::{fmt::Display, time};

use my_term::color::{Background, Foreground};
use serde::{Deserialize, Serialize};

use crate::engine::render::Glyph;
use crate::engine::types::Position3D;

pub type Position = Position3D<i32>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Sprite {
    Static(Static),
    Dynamic(Dynamic),
}
impl Sprite {
    pub fn new_static(sprite: Glyph, pos: Position) -> Self {
        Self::Static(Static {
            pos,
            base: Base { sprite },
        })
    }

    pub fn new_dynamic(sheet: Vec<Glyph>, pos: Position, tick_rate: Duration) -> Self {
        let mut frames = Vec::new();
        for s in sheet {
            frames.push(Base { sprite: s });
        }
        Self::Dynamic(Dynamic {
            pos,
            cursor: 0,
            tick_rate,
            last_tick: None,
            frames,
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

impl std::fmt::Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static(s) => write!(f, "{}", s.base.sprite),
            Self::Dynamic(d) => write!(f, "{}", d.frames[d.cursor].sprite),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Base {
    pub sprite: Glyph,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Static {
    pub pos: Position,
    pub base: Base,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dynamic {
    pub pos: Position,
    pub cursor: usize,
    pub tick_rate: Duration,
    #[serde(skip)]
    pub last_tick: Option<std::time::Instant>,
    pub frames: Vec<Base>,
}
