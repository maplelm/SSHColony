#![deny(unused_variables)]

use std::sync::{Arc, Weak, atomic::AtomicUsize, mpsc::Sender};

use super::super::render::Canvas;
use super::{
    super::types::*,
    Border,
    style::{Measure, Origin, Size, Style},
};
use crate::engine::enums::RenderSignal;
use crate::engine::render::{Layer, Object, RenderUnitId};
use crate::engine::ui::style::{Align, Justify};
use term::color::{Background, Foreground};

#[allow(unused)]
const CURSOR_OFFSET: usize = 2;
#[allow(unused)]
const TOTAL_OFFSET: usize = CURSOR_OFFSET * 2;

#[derive(Debug)]
pub struct Item<I, O> {
    pub label: String,
    pub action: fn(I) -> O,
}

impl<I, O> Item<I, O> {
    pub fn new(l: String, f: fn(I) -> O) -> Self {
        Self {
            label: l,
            action: f,
        }
    }
}

//////////////
///  MENU  ///
//////////////
#[derive(Debug)]
pub struct Menu<I, O> {
    pub render_id: Weak<RenderUnitId>,
    marker: char,
    style: Style,
    position: Position<i32>,
    #[allow(unused)]
    origin: Origin,

    items: Vec<Item<I, O>>,
    cursor: usize,
    #[allow(unused)]
    max_per_page: u16,
    _page: u16,
}

impl<I, O> Menu<I, O> {
    pub fn new(
        x: i32,
        y: i32,
        w: Option<Measure>,
        h: Option<Measure>,
        origin: Origin,
        justify: Justify,
        align: Align,
        border: Option<Border>,
        items: Vec<Item<I, O>>,
        fg: Option<Foreground>,
        bg: Option<Background>,
    ) -> Self {
        Self {
            render_id: Weak::new(),
            position: Position { x: x + 1, y: y + 1 },
            style: Style {
                size: Size {
                    width: w,
                    height: h,
                },
                border: border,
                justify: justify,
                align: align,
                foreground: fg,
                background: bg,
            },
            origin: origin,
            items: items,
            marker: '>',
            cursor: 0,
            max_per_page: 0,
            _page: 0,
        }
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
                    output += b.get_pad_left() + b.get_pad_right() + b.width();
                }
                output += TOTAL_OFFSET;
                return output;
            }
        }
    }

    pub fn execute(&mut self, i: I) -> O {
        (self.items[self.cursor].action)(i)
    }

    pub fn cursor_pos(&self) -> Position<i32> {
        match &self.style.border {
            None => Position::new(self.x(), self.y() + self.cursor as i32),

            Some(b) => Position::new(
                self.x() + 1 + b.get_pad_left() as i32,
                self.y() + 1 + b.get_pad_top() as i32 + self.cursor as i32,
            ),
        }
    }

    pub fn add(&mut self, item: Item<I, O>) {
        self.items.push(item);
    }

    pub fn cursor_up(&mut self, amount: usize) -> bool {
        if self.cursor as isize - amount as isize >= 0 {
            self.cursor -= amount;
            return true; // Moved
        }
        false // did not move
    }

    pub fn cursor_down(&mut self, amount: usize) -> bool {
        if self.cursor + amount < self.items.len() {
            self.cursor += amount;
            return true; // Moved
        }
        false // did not move
    }

    pub fn output(&mut self, render_tx: &Sender<RenderSignal>) {
        let mut out = String::new();
        for (i, l) in self.items.iter().enumerate() {
            if i == self.cursor {
                out.push(self.marker);
                out.push(' ');
            } else {
                out.push_str("  ");
            }
            out.push_str(&l.label);
            if i < self.items.len() - 1 {
                out.push('\n');
            }
        }
        if let Some(id) = self.render_id.upgrade() {
            if id.load() == 0 {
                // Render had not gotten to initialize this object, skip for now?
            }
        } else {
        }
        match self.render_id.upgrade() {
            None => {
                let arc_id = RenderUnitId::new(Layer::Ui);
                self.render_id = Arc::downgrade(&arc_id);
                render_tx.send(RenderSignal::Insert(
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
                max -= b.get_pad_top() + b.get_pad_bottom() + b.height();
            }
            self.max_per_page = max as u16;
        } else {
            max = canvas.height - self.position.y as usize;
            if let Some(b) = &self.style.border {
                max -= b.get_pad_top() + b.get_pad_bottom() + b.height();
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
