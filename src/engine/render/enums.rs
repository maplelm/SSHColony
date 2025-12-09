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
use super::super::traits::Storeable;
use super::super::types::{self as enginetypes, Position};
use super::{
    super::ui::style::{Align, Justify, Measure, Style},
    Camera, Canvas,
    sprite::{self, Glyph, Sprite},
    text::Textbox,
};
use crate::engine::{types::Position3D, ui::Border};
use my_term::Character;
use my_term::color::{Background, Foreground};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Object {
    Sprite(Sprite),
    Text(Textbox),
}

impl Object {
    pub fn pos(&self) -> ObjPosition {
        match self {
            Self::Sprite(s) => match s {
                Sprite::Static(s) => s.pos,
                Sprite::Dynamic(d) => d.pos,
            },
            Self::Text(t) => match t {
                Textbox::Static(s) => s.pos,
                Textbox::Dynamic(d) => d.pos,
            },
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

    pub fn draw(&mut self, can: &Canvas, cam: &Camera, output: &mut String, reset: &str) {
        if !cam.in_view(self, can) {
            return;
        }

        let pos = cam.get_screen_pos(self.pos());
        let l_offset = if pos.x < 0 { pos.x.abs() } else { 0 };
        let r_offset = if pos.x + self.width(can) as i32 > can.width as i32 {
            pos.x + self.width(can) as i32 - can.width as i32
        } else {
            0
        };
        let t_offset = if pos.y < 0 { pos.y.abs() } else { 0 };
        let b_offset = if pos.y + self.height(can) as i32 > can.height as i32 {
            pos.y + self.height(can) as i32 - can.height as i32
        } else {
            0
        };

        output.push_str(&format!("\x1b[{};{}f", pos.x + l_offset, pos.y + b_offset));
        match self {
            Self::Sprite(s) => match s {
                Sprite::Static(s) => match &s.base.sprite {
                    Glyph::Small(c) => output.push_str(&format!("{}", c)),
                    Glyph::Block(b) => {
                        for (i, l) in b.iter().enumerate() {
                            if i == 0 {
                                output.push_str(&format!(
                                    "{}",
                                    l.slice(l_offset as usize, l.len() - r_offset as usize)
                                ));
                            } else {
                                output.push_str(&format!(
                                    "\x1b[{};{}f{}",
                                    pos.x + l_offset + i as i32,
                                    pos.y + b_offset + i as i32,
                                    l.slice(l_offset as usize, l.len() - r_offset as usize)
                                ));
                            }
                        }
                    }
                },
                Sprite::Dynamic(d) => match &d.frames[d.cursor].sprite {
                    Glyph::Small(c) => output.push_str(&format!("{}", c)),
                    Glyph::Block(b) => {
                        for (i, l) in b.iter().enumerate() {
                            if i == 0 {
                                output.push_str(&format!(
                                    "{}",
                                    l.slice(l_offset as usize, l.len() - r_offset as usize)
                                ));
                            } else {
                                output.push_str(&format!(
                                    "\x1b[{};{}f{}",
                                    pos.x + l_offset + i as i32,
                                    pos.y + b_offset + i as i32,
                                    l.slice(l_offset as usize, l.len() - r_offset as usize)
                                ));
                            }
                        }
                    }
                },
            },
            Self::Text(t) => match t {
                Textbox::Static(s) => {
                    for (i, l) in s
                        .base
                        .slice(
                            t_offset as usize,
                            b_offset as usize,
                            l_offset as usize,
                            r_offset as usize,
                        )
                        .lines
                        .iter()
                        .enumerate()
                    {
                        if i == 0 {
                            output.push_str(&format!("{l}"));
                        } else {
                            output.push_str(&format!(
                                "\x1b[{};{}f{}",
                                pos.x + l_offset + i as i32,
                                pos.y + b_offset + i as i32,
                                l
                            ));
                        }
                    }
                }
                Textbox::Dynamic(d) => {
                    for (i, l) in d.frames[d.cursor]
                        .slice(
                            t_offset as usize,
                            b_offset as usize,
                            l_offset as usize,
                            r_offset as usize,
                        )
                        .lines
                        .iter()
                        .enumerate()
                    {
                        if i == 0 {
                            output.push_str(&format!("{l}"));
                        } else {
                            output.push_str(&format!(
                                "\x1b[{};{}f{}",
                                pos.x + l_offset + i as i32,
                                pos.y + b_offset + i as i32,
                                l
                            ));
                        }
                    }
                }
            },
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
