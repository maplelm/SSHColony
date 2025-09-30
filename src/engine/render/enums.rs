use super::{Canvas, drawable::*};
use crate::engine::{
    traits::Storeable,
    types as enginetypes,
    ui::style::{Justify, Measure},
};
use crate::engine::{types::Position, ui::Border};
use std::{
    fmt::Display,
    sync::mpsc,
    time::{Duration, Instant},
};
use term::color::{Background, Foreground};

#[derive(Clone, Debug)]
enum Static {
    Sprite(StaticSprite),
    Text(StaticText),
}

#[derive(Clone, Debug)]
enum Dynamic {
    Sprite(DynamicSprite),
    Text(DynamicText),
}

#[derive(Clone, Debug)]
pub enum Object {
    Static(Static),
    Dynamic(Dynamic),
}

impl Object {
    pub fn id(&self) -> u32 {
        match self {
            Self::Static(s) => match s {
                Static::Sprite(s) => s.id,
                Static::Text(t) => t.id,
            },
            Self::Dynamic(d) => match d {
                Dynamic::Sprite(s) => s.id,
                Dynamic::Text(t) => t.id,
            },
        }
    }
    pub fn pos(&self) -> Position<i32> {
        match self {
            Self::Static(s) => match s {
                Static::Sprite(s) => s.pos,
                Static::Text(t) => t.pos,
            },
            Self::Dynamic(d) => match d {
                Dynamic::Sprite(s) => s.pos,
                Dynamic::Text(t) => t.pos,
            },
        }
    }

    pub fn to_string(&self, canvas: &Canvas) -> String {
        match self {
            Self::Static(s) => match s {
                Static::Sprite(s) => s.to_string(),
                Static::Text(t) => t.to_string(canvas),
            },
            Self::Dynamic(d) => match d {
                Dynamic::Sprite(s) => s.to_string(),
                Dynamic::Text(t) => t.to_string(canvas),
            },
        }
    }

    pub fn as_static_sprite(
        pos: Position<i32>,
        sym: char,
        layer: Layer,
        fg: Option<Foreground>,
        bg: Option<Background>,
    ) -> Self {
        Self::Static(Static::Sprite(StaticSprite {
            pos: pos,
            base: SpriteBase {
                symbol: sym,
                layer: layer,
                fg: fg,
                bg: bg,
            },
        }))
    }

    pub fn as_dyn_sprite(pos: Position<i32>, tick: Duration, sheet: Vec<SpriteBase>) -> Self {
        Self::Dynamic(Dynamic::Sprite(DynamicSprite {
            pos: pos,
            sprite_sheet: sheet,
            tick: tick,
            cursor: 0,
            last_tick: Instant::now(),
        }))
    }

    pub fn as_static_text(
        pos: Position<i32>,
        text: String,
        layer: Layer,
        justify: Justify,
        width: Option<Measure>,
        height: Option<Measure>,
        border: Option<Border>,
        fg: Option<Foreground>,
        bg: Option<Background>,
    ) -> Self {
        Self::Static(Static::Text(StaticText {
            pos: pos,
            base: TextBase {
                text: text,
                layer: layer,
                justify: justify,
                width: width,
                height: height,
                border: border,
                fg: fg,
                bg: bg,
            },
        }))
    }

    pub fn as_dyn_text(pos: Position<i32>, sheet: Vec<TextBase>, tick: Duration) -> Self {
        Self::Dynamic(Dynamic::Text(DynamicText {
            pos: pos,
            text_sheet: sheet,
            tick: tick,
            cursor: 0,
            last_tick: Instant::now(),
        }))
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            Object::Static(_) => false,
            Object::Dynamic(_) => true,
        }
    }

    pub fn update(&mut self) -> bool {
        match self {
            Self::Static(_) => false,
            Self::Dynamic(d) => match d {
                Dynamic::Sprite(s) => s.update(),
                Dynamic::Text(t) => t.update(),
            },
        }
    }
}

pub enum Msg {
    Insert(Object),
    Background(Background),
    Foreground(Foreground),
    Remove(enginetypes::Position<usize>),
    Swap(enginetypes::Position<usize>, enginetypes::Position<usize>),
    Batch(Vec<Msg>),
    TermSizeChange(u32, u32),
    Clear,
}

#[derive(
    Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Layer {
    Background,
    Middleground,
    Foreground,
}

