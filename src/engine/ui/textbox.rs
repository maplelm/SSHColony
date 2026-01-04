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
    render::{Canvas, Char, Layer, Object, ObjectData, RenderUnitId, Text, TextType},
    types::Position,
    ui::style::Measure,
};
use std::sync::{
    Arc, Weak,
    mpsc::{SendError, Sender},
};

#[derive(Debug)]
pub struct TextArea {
    pub render_id: Weak<RenderUnitId>,
    position: Position<i32>,
    marker: Char,
    style: Style,
    value: Vec<Text>,
    place_holder: Option<String>,
    cursor: Position<usize>,
}

impl TextArea {
    pub fn new(
        x: i32,
        y: i32,
        marker: char,
        style: Style,
        placeholder: Option<String>,
    ) -> Self {
        Self {
            render_id: Weak::new(),
            position: Position { x: x, y: y },
            marker: Char::new(marker, style.fg(), style.bg()),
            value: vec![Text::from("", style.fg(), style.bg())],
            style,
            place_holder: placeholder,
            cursor: Position {x: 0, y: 0},
        }
    }

    pub fn max_len_value(&self, canvas: &Canvas) -> usize {
        if let Some(w) = self.style.size.width {
            if let Some(b) = self.style.border.as_ref() {
                w.get(canvas.width) as usize
                    - (self.style.border.as_ref().unwrap().width()
                        + self.style.border.as_ref().unwrap().l_pad()
                        + self.style.border.as_ref().unwrap().r_pad()
                        + 1) as usize
            } else {
                w.get(canvas.width) as usize - 1
            }
        } else {
            canvas.width - (self.position.x as usize + 1)
        }
    }

    pub fn get_value(&self) -> Vec<String> {
        let mut v = vec![];
        for each in self.value.iter() {
            v.push(each.to_string());
        }
        v
    }

    pub fn process_key(
        &mut self,
        key: KeyEvent,
        render_tx: &Sender<RenderSignal>,
        canvas: &Canvas,
    ) {
        let mut dirty = true;
        match key {
            KeyEvent::Backspace => {
                let x = self.cursor.x;
                let y = self.cursor.y;
                let rows = self.value.len();
                let cols = self.value[y].len();

                if rows == 1 {
                    if x == cols && cols > 0{
                        self.value[0].pop();
                        self.cursor.x -= 1;
                    } else if x != 0 {
                        self.value[0].remove(x);
                        self.cursor.x -= 1;
                    }
                } else {
                    if x == 0 && y == 0 {

                    } else if x != 0 && x == cols && cols > 0{
                        self.value[y].pop();
                        self.cursor.x -= 1;
                    } else if x != 0 && x != cols {
                        self.value[y].remove(x);
                        self.cursor.x -= 1;
                    } else if x == 0 && y != 0 {
                        let val = self.value.remove(y);
                        self.value[y-1].join(val);
                        self.cursor.x = self.value[y].len();
                    }
                }
            }
            KeyEvent::Char(c) => {
                if self.max_len_value(canvas) > self.value.len() {
                    let x = self.cursor.x;
                    let y = self.cursor.y;
                    let rows = self.value.len();
                    let cols = self.value[y].len();

                    if x == cols {
                        self.value[y].push(Char::new(c, self.style.fg(), self.style.bg()));
                    } else {
                        self.value[y].insert(x, Char::new(c, self.style.fg(), self.style.bg()));
                    }
                    self.cursor.x += 1;
                }
            }
            KeyEvent::Enter => {
                let x = self.cursor.x;
                let y = self.cursor.y;
                let rows = self.value.len();
                let cols = self.value[y].len();
                if x < cols {
                    let (l, r) = self.value[y].clone().split(x);
                    self.value[y] = l;
                    self.value.insert(y, r);
                    self.cursor.y += 1;
                    self.cursor.x = 0;
                } else if x >= cols {
                    self.value.insert(y, Text::new());
                    self.cursor.y +=1;
                    self.cursor.x = 0;
                }
            }
            KeyEvent::Up => {
                if self.cursor.y > 0 {
                    self.cursor.y -= 1;
                    if self.cursor.x > self.value[self.cursor.y].len() {
                        self.cursor.x = self.value[self.cursor.y].len();
                    }
                }

            }
            KeyEvent::Down => {
                if self.cursor.y < self.value.len() {
                    self.cursor.y += 1;
                    if self.cursor.x > self.value[self.cursor.y].len() {
                        self.cursor.x = self.value[self.cursor.y].len();
                    }
                }
            }
            KeyEvent::Left => {
                if self.cursor.x > 0 {
                    self.cursor.x -= 1;
                }
            }
            KeyEvent::Right => {
                if self.cursor.x < self.value[self.cursor.y].len() {
                    self.cursor.x += 1
                }
            }
            _ => {
                dirty = false;
            }
        }
        if dirty {
            self.output(render_tx, canvas);
        }
    }

    pub fn output(
        &mut self,
        render_tx: &Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> Result<(), SendError<RenderSignal>> {
        let mut out = self.value.clone();
        out[self.cursor.y].insert(self.cursor.x, self.marker.clone());
        match self.render_id.upgrade() {
            None => {
                let arc_id = RenderUnitId::new(Layer::Ui);
                self.render_id = Arc::downgrade(&arc_id);
                render_tx.send(RenderSignal::Insert(
                    arc_id,
                    ObjectData::Text { pos: self.position.clone().into(), data: TextType::Single(out), style: self.style.clone() },
                ))
            }
            Some(arc) => render_tx.send(RenderSignal::Update(
                arc,
                ObjectData::Text {pos: self.position.clone().into(), data: TextType::Single(out), style: self.style.clone()}
            )),
        }
    }
}
