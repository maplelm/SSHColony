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

#![deny(unused_variables)]

use crate::engine::render::{Char, ObjectData, PushChar, PushText, Text, TextType};
use std::sync::mpsc;
use std::sync::{Arc, Weak, atomic::AtomicUsize, mpsc::Sender};

use super::super::render::{Canvas, RenderQueue};
use super::{
    super::types::*,
    Border,
    style::{Measure, Size, Style},
};
use crate::engine::enums::RenderSignal;
use crate::engine::render::{Layer, Object, RenderUnitId};
use crate::engine::ui::style::{Align, Justify};
use my_term::color::{Background, Foreground};

#[allow(unused)]
const CURSOR_OFFSET: usize = 2;
#[allow(unused)]
const TOTAL_OFFSET: usize = CURSOR_OFFSET * 2;

#[derive(Debug)]
pub struct Item<O> {
    pub label: Text,
    pub action: fn() -> O,
}

impl<O> Item<O> {
    pub fn new(l: Text, f: fn() -> O) -> Self {
        Self {
            label: l,
            action: f,
        }
    }

    pub fn lines(matrix: &Vec<Self>) -> Vec<Text> {
        let mut v = vec![];
        for each in matrix {
            v.push(each.label.clone());
        }
        v
    }
}

impl<O> std::fmt::Display for Item<O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

//////////////
///  MENU  ///
//////////////
#[derive(Debug)]
pub struct Menu<O> {
    pub render_id: Weak<RenderUnitId>,
    pub render_queue: RenderQueue,
    marker: Char,
    style: Style,
    position: Position<i32>,
    items: Vec<Item<O>>,
    cursor: usize,
    #[allow(unused)]
    max_per_page: u16,
    _page: u16,
}

impl<O> Menu<O> {
    pub fn new(
        x: i32,
        y: i32,
        render_queue: RenderQueue,
        style: Style,
        items: Vec<Item<O>>,
    ) -> Self {
        Self {
            render_id: Weak::new(),
            render_queue,
            position: Position { x, y },
            marker: Char::new('>', style.fg(), style.bg()),
            style,
            items: items,
            cursor: 0,
            max_per_page: 0,
            _page: 0,
        }
    }

    pub fn shift(&mut self, x: i32, y: i32) {
        self.position.x += x;
        self.position.y += y;
        self.output();
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn x(&self) -> i32 {
        self.position.x
    }
    pub fn y(&self) -> i32 {
        self.position.y
    }
    pub fn width(&self, canvas: &Canvas) -> usize {
        match &self.style.size.width {
            // Specified width
            Some(w) => match *w {
                Measure::Cell(w) => w as usize,
                Measure::Percent(w) => ((canvas.width as f64 / 100.0) * w as f64) as usize,
            },
            // Calculate width based on length of line items
            None => {
                let mut output = 0;
                output += self.largest_line();
                if let Some(b) = &self.style.border {
                    output += b.width();
                }
                output += TOTAL_OFFSET;
                return output;
            }
        }
    }

    pub fn execute(&mut self) -> O {
        (self.items[self.cursor].action)()
    }

    pub fn cursor_pos(&self) -> Position<i32> {
        match &self.style.border {
            None => Position::new(self.x(), self.y() + self.cursor as i32),

            Some(b) => Position::new(
                self.x() + 1 + b.l_pad() as i32,
                self.y() + 1 + b.top_pad() as i32 + self.cursor as i32,
            ),
        }
    }

    pub fn add(&mut self, item: Item<O>) {
        self.items.push(item);
        self.output();
    }

    pub fn cursor_up(&mut self, amount: usize) -> bool {
        if self.cursor as isize - amount as isize >= 0 {
            self.cursor -= amount;
            self.output();
            return true; // Moved
        }
        false // did not move
    }

    pub fn cursor_down(&mut self, amount: usize) -> bool {
        if self.cursor + amount < self.items.len() {
            self.cursor += amount;
            self.output();
            return true; // Moved
        }
        false // did not move
    }

    pub fn output(&mut self) {
        let mut out = Vec::with_capacity(self.items.len());
        for (i, l) in self.items.iter().enumerate() {
            if i == self.cursor {
                let mut l = l.label.clone();
                l.insert(0, self.marker);
                l.insert(1, Char::new(' ', self.style.fg(), self.style.bg()));
                out.push(l);
            } else {
                let mut l = l.label.clone();
                l.insert(0, Char::new(' ', self.style.fg(), self.style.bg()));
                l.insert(0, Char::new(' ', self.style.fg(), self.style.bg()));
                out.push(l);
            }
        }
        match self.render_id.upgrade() {
            None => {
                let arc_id = RenderUnitId::new(Layer::Ui);
                self.render_id = Arc::downgrade(&arc_id);

                self.render_queue.send(RenderSignal::Insert(
                    arc_id,
                    ObjectData::Text {
                        pos: self.position.clone().into(),
                        data: TextType::Single(out),
                        style: self.style.clone(),
                    },
                ));
            }
            Some(arc) => {
                self.render_queue.send(RenderSignal::Update(
                    arc,
                    ObjectData::Text {
                        pos: self.position.clone().into(),
                        data: TextType::Single(out),
                        style: self.style.clone(),
                    },
                ));
            }
        }
    }

    fn largest_line(&self) -> usize {
        if self.items.len() == 0 {
            return 0;
        }
        let mut max = self.items[0].label.len();
        for each in self.items.iter() {
            if each.label.len() > max {
                max = each.label.len();
            }
        }
        return max;
    }

    fn update_max_per_page(&mut self, canvas: &Canvas) {
        let mut max: usize;
        if let Some(h) = &self.style.size.height {
            max = h.get(canvas.height - self.position.y as usize);
            if let Some(b) = &self.style.border {
                max -= b.height();
            }
            self.max_per_page = max as u16;
        } else {
            max = canvas.height - self.position.y as usize;
            if let Some(b) = &self.style.border {
                max -= b.height();
            }
            self.max_per_page = max as u16;
        }
    }
}

/*
#[cfg(test)]
mod test {
    use crate::engine::ui::{BorderSprite, Padding};

    use super::*;

    #[derive(Debug)]
    struct TestItem {
        name: String
    }

    impl TestItem {
        pub fn new(name: String) -> Box<Self> {
            Box::new(Self {name: name})
        }
    }

    impl MenuItem for TestItem {
        fn execute(&mut self) -> bool {
            false
        }
        fn label(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn dynamic_length_menu() {
        let s1 = String::from("testing");
        let s2 = String::from("testing again");
        let s3 = String::from("testing and again");
        let c = Canvas::new(100, 7);
        let mut m = Menu::new(
            0,
            0,
            None,
            None,
            Origin::TopLeft,
            Justify::Left,
            Some(Border::from(
                BorderSprite::String(String::from("#~=")),
                Padding::square(1),
            )),
            vec![
                TestItem::new(s1),
                TestItem::new(s2),
                TestItem::new(s3.clone()),
            ],
        );
        let output = match m.output(&c) {
            Some(out) => out,
            None => "".to_string(),
        };
        println!("{}", output);
        for each in output.split('\n') {
            assert_eq!(each.len(), s3.len() + 8);
        }
    }

    #[test]
    fn fixed_length_menu() {
        let c = Canvas::new(40, 7);
        let mut m = Menu::new(
            0,
            0,
            Some(Measure::Percent(100)),
            Some(Measure::Percent(100)),
            Origin::TopLeft,
            Justify::Left,
            Some(Border::from(
                BorderSprite::String(String::from("#~=")),
                Padding::square(1),
            )),
            vec![
                TestItem::new("testing".to_string()),
                TestItem::new("testing again".to_string()),
                TestItem::new("testing and again".to_string()),
            ],
        );
        let out = match m.output(&c) {
            Some(out) => out,
            None => "".to_string(),
        };
        println!("{}", out);
        for each in out.split('\n') {
            assert_eq!(each.len(), c.width);
        }
    }
}
*/
