use super::{Canvas, drawable::*};
use crate::engine::{
    traits::Storeable,
    types as enginetypes,
    ui::style::{Justify, Measure},
};
use crate::engine::{types::Position3D, ui::Border};
use std::{
    fmt::Display,
    sync::{mpsc, atomic::AtomicUsize},
    time::{Duration, Instant},
};
use term::color::{Background, Foreground};

#[derive(Clone, Debug)]
pub enum Static {
    Sprite(StaticSprite),
    Text(StaticText),
}

#[derive(Clone, Debug)]
pub enum Dynamic {
    Sprite(DynamicSprite),
    Text(DynamicText),
}

#[derive(Clone, Debug)]
pub enum Object {
    Static(Static),
    Dynamic(Dynamic),
}

impl Object {
    pub fn pos(&self) -> Position3D<i32> {
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

    pub fn shift(&mut self, pos: Position3D<i32>) {
        match self {
            Self::Static(s) => match s {
                Static::Sprite(s) => {
                    s.pos.x += pos.x;
                    s.pos.y += pos.y;
                    s.pos.z += pos.z;
                }
                Static::Text(t) => {
                    t.pos.x += pos.x;
                    t.pos.y += pos.y;
                    t.pos.z += pos.z;
                }
            },
            Self::Dynamic(d) => match d {
                Dynamic::Sprite(s) => {
                    s.pos.x += pos.x;
                    s.pos.y += pos.y;
                    s.pos.z += pos.z;
                }
            }
        }
    }

    pub fn layer(&self) -> &Layer {
        match self {
            Self::Static(s) => match s {
                Static::Sprite(s) => &s.layer,
                Static::Text(t) => &t.layer,
            }
            Self::Dynamic(d) => match d {
                Dynamic::Sprite(s) => &s.layer,
                Dynamic::Text(t) => &t.layer
            }
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
        pos: Position3D<i32>,
        sym: char,
        layer: Layer,
        fg: Option<Foreground>,
        bg: Option<Background>,
    ) -> Self {
        Self::Static(Static::Sprite(StaticSprite {
            pos: pos,
            layer: layer,
            base: SpriteBase {
                symbol: sym,
                fg: fg,
                bg: bg,
            },
        }))
    }

    pub fn as_dyn_sprite(pos: Position3D<i32>, layer: Layer, tick: Duration, sheet: Vec<SpriteBase>) -> Self {
        Self::Dynamic(Dynamic::Sprite(DynamicSprite {
            pos: pos,
            layer: layer,
            sprite_sheet: sheet,
            tick: tick,
            cursor: 0,
            last_tick: Instant::now(),
        }))
    }

    pub fn as_static_text(
        pos: Position3D<i32>,
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
            layer: layer,
            base: TextBase {
                text: text,
                justify: justify,
                width: width,
                height: height,
                border: border,
                fg: fg,
                bg: bg,
            },
        }))
    }

    pub fn as_dyn_text(pos: Position3D<i32>, layer: Layer, sheet: Vec<TextBase>, tick: Duration) -> Self {
        Self::Dynamic(Dynamic::Text(DynamicText {
            pos: pos,
            layer: layer,
            text_sheet: sheet,
            tick: tick,
            cursor: 0,
            last_tick: Instant::now(),
        }))
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Dynamic(_) => true,
            _ => false
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Self::Static(_) => true,
            _ => false
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


#[derive(
    Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Layer {
    Background,
    Middleground,
    Foreground,
}

pub enum RenderUnitId {
    Background(AtomicUsize),
    Middleground(AtomicUsize),
    Foreground(AtomicUsize)
}

impl RenderUnitId {
    pub fn from_usize(val: usize, layer: Layer) -> Self {
        let val = AtomicUsize::new(val);
        match layer {
            Layer::Background => RenderUnitId::Background(val),
            Layer::Middleground => RenderUnitId::Middleground(val),
            Layer::Foreground => RenderUnitId::Foreground(val)
        }
    }

    pub fn from_atomic(val: AtomicUsize, layer: Layer) -> Self {
        match layer {
            Layer::Background => RenderUnitId::Background(val),
            Layer::Middleground => RenderUnitId::Middleground(val),
            Layer::Foreground => RenderUnitId::Foreground(val)
        }
    }

    pub fn is_bg(&self) -> bool {
        match self {
            RenderUnitId::Background(_) => true,
            _ => false
        }
    }

    pub fn is_mg(&self) -> bool {
        match self {
            RenderUnitId::Middleground(_) => true,
            _ => false
        }
    }

    pub fn is_fg(&self) -> bool {
        match self {
            RenderUnitId::Foreground(_) => true,
            _ => false
        }
    }

    pub fn load(&self) -> (usize, Layer) {
        match self {
            Self::Background(val) => (val.load(std::sync::atomic::Ordering::SeqCst), Layer::Background),
            Self::Middleground(val) => (val.load(std::sync::atomic::Ordering::SeqCst), Layer::Middleground),
            Self::Foreground(val) => (val.load(std::sync::atomic::Ordering::SeqCst), Layer::Foreground),
        }
    }

    pub fn store(&self, val: usize) {
        match self {
            Self::Background(aval) => aval.store(val, std::sync::atomic::Ordering::SeqCst),
            Self::Middleground(aval) => aval.store(val, std::sync::atomic::Ordering::SeqCst),
            Self::Foreground(aval) => aval.store(val, std::sync::atomic::Ordering::SeqCst),
        }
    }

}