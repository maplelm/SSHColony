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

use crate::engine::ui::style::{Coloring, Style};
use crate::engine::{enums::RenderSignal, types::Position};
use std::sync::{Arc, Weak, atomic::AtomicUsize, mpsc::Sender};

use super::{
    Border, BorderSprite,
    style::{Align, Justify, Measure},
};
use crate::engine::render::{Layer, Object, ObjectData, RenderUnitId, Text, TextType, Textbox};
use my_term::color::{Background, Foreground};

pub struct Button<I, O> {
    render_id: Weak<RenderUnitId>,
    text: Vec<Text>,
    style: Style,
    pos: Position<i32>,
    select_color: Coloring,
    selected: bool,
    action: fn(I) -> O,
}

impl<I, O> Button<I, O> {
    pub fn new(
        pos: Position<i32>,
        text: Vec<Text>,
        style: Style,
        select_color: Coloring,
        action: fn(I) -> O,
    ) -> Self {
        Self {
            render_id: Weak::new(),
            text,
            pos,
            style,
            select_color,
            selected: false,
            action: action,
        }
    }

    pub fn output(&mut self, render_tx: &Sender<RenderSignal>) {
        let (fg, bg) = self.current_colors();
        match self.render_id.upgrade() {
            Some(arc_id) => {
                render_tx.send(RenderSignal::Update(
                    arc_id,
                    ObjectData::Text { pos: self.pos.clone().into(), data: TextType::Single(self.text.clone()), style:self.style.clone() }
                ));
            }
            None => {
                let arc_id = RenderUnitId::new(Layer::Ui);
                self.render_id = Arc::downgrade(&arc_id);
                render_tx.send(RenderSignal::Insert(
                    arc_id,
                    ObjectData::Text { pos: self.pos.clone().into(), data: TextType::Single(self.text.clone()), style: self.style.clone() },
                ));
            }
        }
    }

    fn current_colors(&self) -> (Foreground, Background) {
        if self.selected {
            (self.select_color.foreground.clone(), self.select_color.background.clone())
        } else {
            (self.style.fg().clone(), self.style.bg().clone())
        }
    }

    pub fn toggle_select(&mut self, render_tx: &Sender<RenderSignal>) {
        self.selected = !self.selected;
        self.output(render_tx);
    }

    pub fn execute(&self, input: I) -> O {
        (self.action)(input)
    }
}
