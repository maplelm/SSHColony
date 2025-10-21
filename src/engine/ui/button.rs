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

use crate::engine::ui::style::Style;
use crate::engine::{enums::RenderSignal, types::Position};
use std::sync::{Arc, Weak, atomic::AtomicUsize, mpsc::Sender};

use super::{
    Border, BorderSprite,
    style::{Align, Justify, Measure},
};
use crate::engine::render::{Layer, Object, RenderUnitId};
use term::color::{Background, Foreground};

pub struct Button<I, O> {
    render_id: Weak<RenderUnitId>,
    text: String,
    pos: Position<i32>,
    style: Style,
    select_foreground: Option<Foreground>,
    select_background: Option<Background>,
    selected: bool,
    action: fn(I) -> O,
}

impl<I, O> Button<I, O> {
    pub fn new(
        render_id: Weak<RenderUnitId>,
        text: String,
        x: i32,
        y: i32,
        style: Option<Style>,
        sfg: Option<Foreground>,
        sbg: Option<Background>,
        action: fn(I) -> O,
    ) -> Self {
        Self {
            render_id: Weak::new(),
            text: text,
            pos: Position { x: x, y: y },
            style: if let Some(s) = style {
                s
            } else {
                Style::default()
            },
            select_foreground: sfg,
            select_background: sbg,
            selected: false,
            action: action,
        }
    }

    pub fn output(&mut self, render_tx: &Sender<RenderSignal>) {
        let (fg, bg) = self.color_output_init();
        match self.render_id.upgrade() {
            Some(arc_id) => {
                render_tx.send(RenderSignal::Update(
                    arc_id,
                    Object::static_text(
                        self.pos.as_3d(0),
                        self.text.clone(),
                        self.style.justify,
                        self.style.align,
                        self.style.size.width,
                        self.style.size.height,
                        self.style.border.clone(),
                        fg,
                        bg,
                    ),
                ));
            }
            None => {
                let arc_id = RenderUnitId::new(Layer::Ui);
                self.render_id = Arc::downgrade(&arc_id);
                render_tx.send(RenderSignal::Insert(
                    arc_id,
                    Object::static_text(
                        self.pos.as_3d(0),
                        self.text.clone(),
                        self.style.justify,
                        self.style.align,
                        self.style.size.width,
                        self.style.size.height,
                        self.style.border.clone(),
                        fg,
                        bg,
                    ),
                ));
            }
        }
    }

    fn color_output_init(&self) -> (Option<Foreground>, Option<Background>) {
        if self.selected {
            (self.select_foreground, self.select_background)
        } else {
            (self.style.foreground, self.style.background)
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
