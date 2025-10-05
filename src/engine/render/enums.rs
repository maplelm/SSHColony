use super::{Canvas, drawable::*};
use crate::engine::{
    traits::Storeable,
    types as enginetypes,
    ui::style::{Align, Justify, Measure},
};
use crate::engine::{types::Position3D, ui::Border};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    sync::{atomic::AtomicUsize, mpsc},
    time::{Duration, Instant},
};
use term::color::{Background, Foreground};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Static {
    Sprite(StaticSprite),
    Text(StaticText),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Dynamic {
    Sprite(DynamicSprite),
    Text(DynamicText),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Ui {
    Menu,
    TextBox,
    CheckBox,
    RadioButtons,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

    pub fn has_color(&self) -> bool {
        match self {
            Self::Static(s) => match s {
                Static::Sprite(s) => s.base.fg.is_some() || s.base.bg.is_some(),
                Static::Text(t) => t.base.fg().is_some() || t.base.bg().is_some(),
            },
            Self::Dynamic(d) => match d {
                Dynamic::Sprite(s) => {
                    s.sprite_sheet[s.cursor].fg.is_some() || s.sprite_sheet[s.cursor].bg.is_some()
                }
                Dynamic::Text(t) => {
                    t.text_sheet[t.cursor].fg().is_some() || t.text_sheet[t.cursor].bg().is_some()
                }
            },
        }
    }

    pub fn is_sprite(&self) -> bool {
        match self {
            Self::Static(s) => match s {
                Static::Sprite(_) => true,
                _ => false,
            },
            Self::Dynamic(d) => match d {
                Dynamic::Sprite(_) => true,
                _ => false,
            },
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            Self::Static(s) => match s {
                Static::Text(_) => true,
                _ => false,
            },
            Self::Dynamic(d) => match d {
                Dynamic::Text(_) => true,
                _ => false,
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
                Dynamic::Text(t) => {
                    t.pos.x += pos.x;
                    t.pos.y += pos.y;
                    t.pos.z += pos.z;
                }
            },
        }
    }

    pub fn as_str(&mut self, canvas: &Canvas) -> &str {
        match self {
            Self::Static(s) => match s {
                Static::Sprite(s) => s.as_str(),
                Static::Text(t) => t.as_str(canvas),
            },
            Self::Dynamic(d) => match d {
                Dynamic::Sprite(s) => s.as_str(),
                Dynamic::Text(t) => t.as_str(canvas),
            },
        }
    }

    pub fn static_sprite(
        pos: Position3D<i32>,
        sym: char,
        fg: Option<Foreground>,
        bg: Option<Background>,
    ) -> Self {
        Self::Static(Static::Sprite(StaticSprite {
            pos: pos,
            base: SpriteBase::new(sym, fg, bg),
        }))
    }

    pub fn dyn_sprite(pos: Position3D<i32>, tick: Duration, sheet: Vec<SpriteBase>) -> Self {
        Self::Dynamic(Dynamic::Sprite(DynamicSprite {
            pos: pos,
            sprite_sheet: sheet,
            tick: tick,
            cursor: 0,
            last_tick: Instant::now(),
        }))
    }

    pub fn static_text(
        pos: Position3D<i32>,
        text: String,
        justify: Justify,
        align: Align,
        width: Option<Measure>,
        height: Option<Measure>,
        border: Option<Border>,
        fg: Option<Foreground>,
        bg: Option<Background>,
    ) -> Self {
        Self::Static(Static::Text(StaticText {
            pos: pos,
            base: TextBase::new(text, justify, align, width, height, border, fg, bg),
        }))
    }

    pub fn dyn_text(pos: Position3D<i32>, sheet: Vec<TextBase>, tick: Duration) -> Self {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectTemplate {
    name: String,
    data: Object,
}

impl Storeable for ObjectTemplate {
    type Key = String;
    fn key(&self) -> Self::Key {
        self.name.clone()
    }
}

#[derive(
    Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Layer {
    Background,
    Middleground,
    Foreground,
    Ui,
}
