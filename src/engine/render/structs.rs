#[deny(unused)]

use super::sprites::Sprite;
use std::time::{Duration, Instant};

#[derive(Clone, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct Static {
    pub sprite: Sprite,
}
#[derive(Clone, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct Dynamic {
    pub sprite: Vec<Sprite>,
    pub cursor: usize,
    pub tick: Duration,
    #[serde(skip_serializing, skip_deserializing, default = "Instant::now")]
    pub last_tick: Instant,
}

#[derive(Clone, Hash, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

