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

use super::super::enums::RenderSignal;
use super::Border;
use crate::engine::render::{Char, ObjectData, TextType};
use crate::engine::types::Position;
use crate::engine::ui::style::Style;
use crate::engine::{
    render::{Canvas, Layer, Object, RenderUnitId, Text},
    ui::style::{Coloring, Measure},
};
use my_term::color::{Background, Foreground};
use std::sync::mpsc::SendError;
use std::sync::{Arc, Weak, mpsc::Sender};

#[derive(Debug)]
pub struct SelectorItem {
    pub label: Text,
    pub value: usize,
}

impl SelectorItem {
    pub fn new(l: Text, v: usize) -> Self {
        Self { label: l, value: v }
    }
}

#[derive(Debug)]
pub enum SelectionDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub enum SelectorState {
    None,
    Hovered,
    Selected,
}

#[derive(Debug)]
pub struct Selector {
    pub render_id: Weak<RenderUnitId>,
    pos: Position<i32>,
    style: Style,
    items: Vec<SelectorItem>,
    state: SelectorState,
    select_color: Coloring,
    hover_color: Coloring,
    direction: SelectionDirection,
    selected: Option<usize>,
    cursor: usize,
}

impl Selector {
    pub fn new(
        x: i32,
        y: i32,
        style: Style,
        select_color: Coloring,
        hover_color: Coloring,
        direction: SelectionDirection,
        items: Vec<SelectorItem>,
    ) -> Self {
        Self {
            render_id: Weak::new(),
            pos: Position { x: x, y: y },
            style,
            select_color,
            hover_color,
            state: SelectorState::None,
            items: items,
            direction: direction,
            cursor: 0,
            selected: None,
        }
    }

    pub fn next(&mut self) {
        if self.selected.is_none() {
            self.cursor = (self.cursor + 1) % self.items.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.selected.is_none() {
            if self.cursor == 0 {
                self.cursor = self.items.len() - 1;
            } else {
                self.cursor -= 1;
            }
        }
    }

/*
    pub fn render_vertically(&mut self) -> String {
        let mut output = String::new();
        for (i, mut each) in self.items.iter().enumerate() {
            if i == self.cursor {
                Selector::selected_item(
                    each.label.as_str(),
                    &mut output,
                    &self.style.color,
                    &self.select_color,
                );
            } else {
                output.push_str(&each.label);
            }
            if i < self.items.len() - 1 {
                output.push_str("\n");
            }
        }
        output
    }

    pub fn render_horizontally(&mut self) -> String {
        let mut output = String::new();
        for (i, each) in self.items.iter().enumerate() {
            if i == self.cursor {
                Selector::selected_item(
                    each.label.as_str(),
                    &mut output,
                    &self.style.color,
                    &self.select_color,
                );
            } else {
                output.push_str(&each.label);
            }
            if i < self.items.len() - 1 {
                output.push_str(" ");
            }
        }
        output
    }
*/
    pub fn output(
        &mut self,
        render_tx: &Sender<RenderSignal>,
    ) -> Result<(), SendError<RenderSignal>> {

        let out: Vec<Text> = match self.direction {
            SelectionDirection::Horizontal => {
                let mut t: Text = Text::from("", self.style.fg(), self.style.bg());
                for each in self.items.iter() {
                    t.join(each.label.clone());
                    t.push(Char::new(' ', self.style.fg(), self.style.bg()));
                }
                vec![t]
            }
            SelectionDirection::Vertical => {
                let mut v = Vec::with_capacity(self.items.len());
                for each in self.items.iter() {
                    v.push(each.label.clone())
                }
                v
            }
        };
        match self.render_id.upgrade() {
            None => {
                let arc_id = RenderUnitId::new(Layer::Ui);
                self.render_id = Arc::downgrade(&arc_id);
                render_tx.send(RenderSignal::Insert(
                    arc_id,
                    ObjectData::Text { pos: self.pos.clone().into(), data: TextType::Single(out), style: self.style.clone() },
                ))
            }
            Some(arc) => render_tx.send(RenderSignal::Update(
                arc,
                ObjectData::Text { pos: self.pos.clone().into(), data: TextType::Single(out), style: self.style.clone()},
            )),
        }
    }

    pub fn toggle_select(&mut self) {
        if self.selected.is_none() {
            self.selected = Some(self.cursor);
        } else {
            self.selected = None;
        }
    }

    pub fn get_selected(&self) -> Option<usize> {
        self.selected
    }
}

/*
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn testing_selector_output() {
        let c: Canvas = Canvas {
            width: 100,
            height: 100,
        };
        let mut s = Selector::new(
            0,
            0,
            Style::default(),
            Coloring::default(),
            Coloring::default(),
            SelectionDirection::Horizontal,
            vec![
                SelectorItem {
                    label: "a".to_string(),
                    value: 0,
                },
                SelectorItem {
                    label: "b".to_string(),
                    value: 0,
                },
                SelectorItem {
                    label: "c".to_string(),
                    value: 0,
                },
            ],
        );

        let (tx, rx) = std::sync::mpsc::channel();
        s.output(&tx);
        let mut text = rx.recv().ok().unwrap();
        assert_eq!(
            text.as_str(&c).unwrap().to_string(),
            "a\n\nb\n\nc".to_string()
        );
    }
}
*/