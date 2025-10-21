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

use super::style::Style;
use crate::engine::{
    enums::RenderSignal,
    input::KeyEvent,
    render::{Layer, Object, RenderUnitId, Canvas},
    types::Position,
    ui::style::{Measure, Origin},
};
use std::sync::{Arc, Weak, mpsc::Sender};

pub struct Textbox {
    pub render_id: Weak<RenderUnitId>,
    position: Position<i32>,
    marker: char,
    style: Style,
    origin: Origin,
    value: String,
    place_holder: Option<String>,
    cursor: usize,
}

impl Textbox {
    pub fn new(
        x: i32,
        y: i32,
        marker: char,
        style: Option<Style>,
        placeholder: Option<String>,
    ) -> Self {
        Self {
            render_id: Weak::new(),
            position: Position { x: x, y: y },
            marker: marker,
            style: if let Some(s) = style {
                s
            } else {
                Style::default()
            },
            origin: Origin::TopLeft,
            value: String::new(),
            place_holder: placeholder,
            cursor: 0,
        }
    }

    pub fn max_len_value(&self, canvas: &Canvas) -> usize  {
        if let Some(w) = self.style.size.width {
            if let Some(b) = self.style.border.as_ref() {
                w.get(canvas.width) as usize - (self.style.border.as_ref().unwrap().width() + self.style.border.as_ref().unwrap().get_pad_left() + self.style.border.as_ref().unwrap().get_pad_right() + 1) as usize
            } else {
                w.get(canvas.width) as usize - 1
            }
        } 
        else {
            canvas.width - (self.position.x as usize + 1)
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn process_key(&mut self, key: KeyEvent, render_tx: &Sender<RenderSignal>, canvas: &Canvas) {
        let mut dirty = true;
        match key {
            KeyEvent::Backspace => if self.value.len() > 0 && self.cursor > 0{
                if self.cursor == self.value.len() {
                    self.value.pop();
                }
                else {
                    self.value.remove(self.cursor);
                }
                self.cursor -= 1;
            },
            KeyEvent::Char(c) => if self.max_len_value(canvas) > self.value.len(){
                if self.cursor == self.value.len() {
                    self.value.push(c);
                }
                else {
                    self.value.insert(self.cursor, c);
                }
                self.cursor += 1;
            },
            KeyEvent::Left => if self.cursor > 0 {
                    self.cursor -= 1;
            },
            KeyEvent::Right => if self.cursor < self.value.len() {
                    self.cursor += 1
            },
            _ => {
                dirty = false;
            }
        }
        if dirty {
            self.output(render_tx);
        }
    }

    pub fn output(&mut self, render_tx: &Sender<RenderSignal>) {
        let mut out = self.value.clone();
        out.push(self.marker);
        match self.render_id.upgrade() {
            None => {
                let arc_id = RenderUnitId::new(Layer::Ui);
                self.render_id = Arc::downgrade(&arc_id);
                let _ = render_tx.send(RenderSignal::Insert(
                    arc_id,
                    Object::static_text(
                        self.position.as_3d(0),
                        out,
                        self.style.justify,
                        self.style.align,
                        self.style.size.width,
                        self.style.size.height,
                        self.style.border.clone(),
                        self.style.foreground,
                        self.style.background,
                    ),
                ));
            }
            Some(arc) => {
                render_tx.send(RenderSignal::Update(
                    arc,
                    Object::static_text(
                        self.position.as_3d(0),
                        out,
                        self.style.justify,
                        self.style.align,
                        self.style.size.width,
                        self.style.size.height,
                        self.style.border.clone(),
                        self.style.foreground,
                        self.style.background,
                    ),
                ));
            }
        }
    }
}
