/*
Copyright 2025 Luke Maple

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
you may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
use crate::engine::render::Text;
use crate::engine::render::text::Base;
use crate::engine::render::text::Static;
use crate::engine::ui::style::Style;
use std::fmt::Write;
use std::io::Write as iowrite;

use super::{
    super::{
        traits::Storeable,
        types::{self as enginetypes, Position, Position3D},
        ui::Border,
    },
    Camera, Canvas, Glyph, Sprite, Textbox, TextboxSlice,
};
use my_term::color::{Background, Foreground};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    io::Lines,
    sync::{atomic::AtomicUsize, mpsc},
    time::{Duration, Instant},
};

type ObjPosition = Position3D<i32>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Ui {
    Menu,
    TextBox,
    CheckBox,
    RadioButtons,
}

#[derive(Debug, Clone)]
pub enum GlyphType {
    Single(Glyph),
    Multi {
        frames: Vec<Glyph>,
        tick_rate: Duration,
    },
}

#[derive(Debug, Clone)]
pub enum TextType {
    Single(Vec<Text>),
    Multi {
        frames: Vec<Base>,
        tick_rate: Duration,
    },
}

#[derive(Debug, Clone)]
pub enum ObjectData {
    Sprite {
        pos: Position3D<i32>,
        glyph: GlyphType,
    },
    Text {
        pos: Position3D<i32>,
        data: TextType,
        style: Style,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Object {
    Sprite(Sprite),
    Text(Textbox),
}

impl Object {
    pub fn new_sprite_static(glyph: Glyph, pos: Position3D<i32>) -> Self {
        Self::Sprite(Sprite::new_static(glyph, pos))
    }

    pub fn new_sprite_dynamic(
        glyphs: Vec<Glyph>,
        pos: Position3D<i32>,
        tick_rate: Duration,
    ) -> Self {
        Self::Sprite(Sprite::new_dynamic(glyphs, pos, tick_rate))
    }

    pub fn new_text_static(
        pos: Position3D<i32>,
        lines: Vec<Text>,
        style: Style,
        can: &Canvas,
    ) -> Self {
        Self::Text(Textbox::new_static(pos, lines, style, can))
    }

    pub fn new_text_dynamic(pos: Position3D<i32>, frames: Vec<Base>, tick_rate: Duration) -> Self {
        Self::Text(Textbox::new_dynamic(pos, frames, tick_rate))
    }

    pub fn from_data(data: impl Into<ObjectData>, canvas: &Canvas) -> Self {
        match data.into() {
            ObjectData::Sprite { pos, glyph } => match glyph {
                GlyphType::Single(c) => Self::Sprite(Sprite::new_static(c, pos)),
                GlyphType::Multi { frames, tick_rate } => {
                    Self::Sprite(Sprite::new_dynamic(frames, pos, tick_rate))
                }
            },
            ObjectData::Text { pos, data, style } => match data {
                TextType::Single(t) => Self::Text(Textbox::new_static(pos, t, style, canvas)),
                TextType::Multi { frames, tick_rate } => {
                    Self::Text(Textbox::new_dynamic(pos, frames, tick_rate))
                }
            },
        }
    }

    pub fn pos(&self) -> ObjPosition {
        match self {
            Self::Sprite(s) => s.pos(),
            Self::Text(t) => t.pos(),
        }
    }

    pub fn width(&self, can: &Canvas) -> usize {
        match self {
            Self::Sprite(s) => s.width(),
            Self::Text(t) => t.width(can),
        }
    }

    pub fn height(&self, can: &Canvas) -> usize {
        match self {
            Self::Sprite(s) => s.height(),
            Self::Text(t) => t.height(can),
        }
    }

    #[deny(unused)]
    pub fn draw(&mut self, can: &Canvas, cam: &Camera, stream: &mut std::io::Stdout, _reset: &str) {
        if !cam.in_view(self, can) {
            return;
        }
        let width: i32 = self.width(can) as i32;
        let height: i32 = self.height(can) as i32;

        let virt_pos: Position<i32> = self.pos().into();
        let scr_pos: Position<i32> = cam.get_screen_pos(self.pos());
        let l_edge: i32 = virt_pos.x;
        let r_edge: i32 = virt_pos.x + width;
        let t_edge: i32 = virt_pos.y;
        let b_edge: i32 = virt_pos.y + height;
        let cam_l_edge: i32 = cam.x();
        let cam_r_edge: i32 = cam.x() + cam.width() as i32;
        let cam_t_edge: i32 = cam.y();
        let cam_b_edge: i32 = cam.y() + cam.height() as i32;

        let l_delta: usize = if l_edge < cam_l_edge {
            (cam_l_edge - l_edge) as usize
        } else {
            0
        };

        let r_delta: usize = if r_edge > cam_r_edge {
            (r_edge - cam_r_edge) as usize
        } else {
            0
        };

        let t_delta: usize = if t_edge < cam_t_edge {
            (cam_t_edge - t_edge) as usize
        } else {
            0
        };

        let b_delta: usize = if b_edge > cam_b_edge {
            (b_edge - cam_b_edge) as usize
        } else {
            0
        };

        let _ = write!(
            stream,
            "\x1b[{};{}fObject Edges (l,r,t,b): {},{},{},{} | Cam Edges (l,r,t,b): {},{},{},{}  |",
            can.height - 3,
            0,
            l_edge,
            r_edge,
            t_edge,
            b_edge,
            cam_l_edge,
            cam_r_edge,
            cam_t_edge,
            cam_b_edge
        );
        let _ = write!(
            stream,
            "\x1b[{};{}fOffsets (l,r,t,b): {},{},{},{} | screen pos (x,y): {},{} | world pos (x,y): {},{}     ",
            can.height - 2,
            0,
            l_delta,
            r_delta,
            t_delta,
            b_delta,
            scr_pos.x,
            scr_pos.y,
            virt_pos.x,
            virt_pos.y
        );
        let cursor_pos: Position<i32> = Position {
            x: scr_pos.x + l_delta as i32,
            y: scr_pos.y + t_delta as i32,
        };
        let _ = write!(
            stream,
            "move cursor (x, y): {},{} |",
            cursor_pos.x, cursor_pos.y,
        );

        let _ = write!(stream, "\x1b[{};{}f", cursor_pos.y, cursor_pos.x,);
        match self {
            Self::Sprite(s) => {
                let _ = write!(stream, "{s}");
            }
            Self::Text(t) => {
                let _ = write!(stream, "{}", t.slice(l_delta, r_delta, t_delta, b_delta));
            }
        }
    }

    pub fn is_sprite(&self) -> bool {
        match self {
            Self::Sprite(_) => true,
            _ => false,
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            Self::Text(_) => true,
            _ => false,
        }
    }

    pub fn move_pos(&mut self, pos: Position3D<i32>) {
        match self {
            Self::Sprite(s) => s.move_pos(pos),
            Self::Text(t) => t.move_pos(pos),
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Sprite(s) => s.is_dynamic(),
            Self::Text(t) => t.is_dynamic(),
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Self::Sprite(s) => s.is_static(),
            Self::Text(t) => t.is_static(),
        }
    }

    pub fn update(&mut self) -> bool {
        match self {
            Self::Sprite(s) => s.update(),
            Self::Text(t) => t.update(),
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
