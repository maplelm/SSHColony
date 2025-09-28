use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use term::color::{Foreground, Background};

use super::{sprites::Sprite, structs::*};
use crate::engine::{traits::Storeable, types as enginetypes};

#[derive(
    Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Object {
    Static {
        name: Option<String>,
        sprite: Sprite,
    },
    Dynamic {
        name: Option<String>,
        sprite: Vec<Sprite>,
        tick: Duration,
        #[serde(skip_serializing, skip_deserializing, default = "Object::new_cursor")]
        cursor: usize,
        #[serde(skip_serializing, skip_deserializing, default = "Instant::now")]
        last_tick: Instant,
    },
}

impl Storeable for Object {
    type Key = String;
    fn key(&self) -> Self::Key {
        match self.name() {
            Some(n) => n.clone(),
            None => "".to_string(),
        }
    }
}

impl Object {
    pub fn name(&self) -> &Option<String> {
        match self {
            Object::Static { name, .. } => name,
            Object::Dynamic { name, .. } => name,
        }
    }
    fn new_cursor() -> usize {
        0
    }

    pub fn as_dynamic(&self) -> Option<&Object> {
        if let Object::Dynamic { .. } = self {
            Some(self)
        } else {
            None
        }
    }
    pub fn as_dynamic_mut(&mut self) -> Option<&mut Object> {
        if let Object::Dynamic { .. } = self {
            Some(self)
        } else {
            None
        }
    }
    pub fn as_static(&self) -> Option<&Object> {
        if let Object::Static { .. } = self {
            Some(self)
        } else {
            None
        }
    }
    pub fn as_static_mut(&mut self) -> Option<&mut Object> {
        if let Object::Static { .. } = self {
            Some(self)
        } else {
            None
        }
    }
    pub fn sprite(&self) -> &Sprite {
        match self {
            Object::Dynamic { sprite, cursor, .. } => &sprite[*cursor],
            Object::Static { sprite, .. } => sprite,
        }
    }
    pub fn new_static(sym: char, bg: Option<Background>, fg: Option<Foreground>) -> Object {
        Object::Static {
            sprite: Sprite::new(sym, bg, fg),
            name: None,
        }
    }
    pub fn new_dynamic(s: Vec<Sprite>, tick: std::time::Duration) -> Option<Object> {
        if s.len() < 1 {
            return None;
        }
        return Some(Object::Dynamic {
            name: None,
            sprite: s,
            cursor: 0,
            tick: tick,
            last_tick: std::time::Instant::now(),
        });
    }
    pub fn is_dynamic(&self) -> bool {
        match self {
            Object::Static { .. } => false,
            Object::Dynamic { .. } => true,
        }
    }

    pub fn update(&mut self) -> bool {
        match self {
            Object::Static { .. } => false,
            Object::Dynamic {
                sprite,
                last_tick,
                cursor,
                tick,
                ..
            } => {
                let now = Instant::now();
                if now.duration_since(*last_tick) >= *tick {
                    if *cursor == sprite.len() - 1 {
                        *cursor = 0;
                    } else {
                        *cursor += 1;
                    }
                    *last_tick = now;
                    return true;
                }
                return false;
            }
        }
    }
    pub fn insert(
        self,
        x: usize,
        y: usize,
        sender: &mpsc::Sender<Msg>,
    ) -> Result<(), mpsc::SendError<Msg>> {
        sender.send(Msg::Insert(enginetypes::Position::new(x, y), self))
    }
}

pub enum Msg {
    Insert(enginetypes::Position<usize>, Object),
    InsertRange {
        start: enginetypes::Position<usize>,
        end: enginetypes::Position<usize>,
        object: Object,
    },
    InsertText {
        pos: enginetypes::Position<usize>,
        text: String,
        prefix: Option<String>,
        suffix: Option<String>,
    },
    Background(Background),
    Foreground(Foreground),
    Remove(enginetypes::Position<usize>),
    RemoveRange(enginetypes::Position<usize>, enginetypes::Position<usize>),
    Swap(enginetypes::Position<usize>, enginetypes::Position<usize>),
    Batch(Vec<Msg>),
    TermSizeChange(u32, u32),
    Clear,
}
