#![deny(unused)]

use crate::engine::render::Object;
use crate::engine::ui::style::Justify;

use super::super::render::Canvas;
use super::{
    super::{render, types::*},
    Border,
    style::{Measure, Origin},
};

const CURSOR_OFFSET: usize = 2;
const TOTAL_OFFSET: usize = CURSOR_OFFSET * 2;

pub trait MenuItem: std::fmt::Debug  {
    fn label(&self) -> &str;
    fn execute(&mut self) -> bool; // return true if there is output to get
}

//////////////
///  MENU  ///
//////////////
#[derive(
    Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Menu {
    x: usize,
    y: usize,
    width:  Option<Measure>,
    height: Option<Measure>,
    origin: Origin,
    pub border: Option<Border>,
    pub _label_style: Option<String>,
    pub marker: Object,
    pub justify: Justify,

    #[serde(skip, default="Vec::new")]
    items: Vec<Box<dyn MenuItem>>,
    cursor: usize,
    max_per_page: u16,
    _page: u16,
}

impl Menu {
    pub fn new(
        x: usize,
        y: usize,
        w: Option<Measure>,
        h: Option<Measure>,
        origin: Origin,
        justify: Justify,
        border: Option<Border>,
        items: Vec<Box<dyn MenuItem>>,
    ) -> Self {
        Self {
            x: x,
            y: y,
            width: w,
            height: h,
            origin: origin,
            items: items,
            border: border,
            _label_style: None,
            justify: justify,
            marker: Object::new_static('>', None, None),
            cursor: 0,
            max_per_page: 0,
            _page: 0,
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    pub fn width(&self, canvas: &Canvas) -> usize {
        match &self.width {
            // Specified width
            Some(w) => match *w {
                Measure::Cell(w) => w as usize,
                Measure::Percent(w) => ((canvas.width as f64 / 100.0) * w as f64) as usize,
            },
            // Calculate width based on length of line items
            None => {
                let mut output = 0;
                output += self.largest_line();
                if let Some(b) = &self.border {
                    output += b.get_pad_left() + b.get_pad_right() + b.width();
                }
                output += TOTAL_OFFSET;
                return output;
            }
        }
    }

    pub fn execute(&mut self) {
        self.items[self.cursor].execute();
    }

    pub fn cursor_pos(&self) -> Position<usize> {
        match &self.border {
            None => Position::new(self.x(), self.y() + self.cursor),

            Some(b) => Position::new(
                self.x() + 1 + b.get_pad_left(),
                self.y() + 1 + b.get_pad_top() + self.cursor,
            ),
        }
    }

    pub fn marker_object(&self) -> render::Object {
        self.marker.clone()
    }

    pub fn add(&mut self, item: Box<dyn MenuItem>) {
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

    pub fn output(&mut self, canvas: &Canvas) -> Option<String> {
        if self.items.len() == 0 {
            return None;
        }
        let mut out = String::new();
        self.update_max_per_page(canvas);
        let width = self.width(canvas);
        let iter = self.render_top_border(&mut out, width);
        let iter = self.render_item(&mut out, width, iter);
        self.render_bottom_border(&mut out, width, iter);
        return Some(out);
    }

    fn render_bottom_border(&self, output: &mut String, width: usize, mut iter: usize) {
        if let Some(b) = &self.border {
            let mut spacing = String::new();
            for _ in 0..width - b.width() {
                spacing.push(' ');
            }
            let mut line = String::new();
            for _ in 0..b.get_pad_bottom() {
                if let Some(l) = b.get_left(iter) {
                    line.push(l);
                }
                line.push_str(&spacing);
                if let Some(r) = b.get_right(iter) {
                    line.push(r);
                }
                line.push('\n');
                iter += 1;
            }

            for i in 0..width {
                if let Some(c) = b.get_bottom(i) {
                    line.push(c)
                }
            }
            output.push_str(&line);
        }
    }

    fn render_top_border(&mut self, output: &mut String, width: usize) -> usize {
        if let Some(b) = &self.border {
            // Generate Top Border
            let mut line = String::new();
            let mut iter: usize = 0;
            while let Some(c) = b.get_top(iter)
                && line.len() < width
            {
                line.push(c);
                iter += 1;
            }
            output.push_str(&line);
            output.push('\n');

            // Generate top padding
            if b.get_pad_top() == 0 {
                return 0;
            }
            line.clear();
            let mut spacing = String::new();
            for _ in 0..width - b.width() {
                spacing.push(' ');
            }

            iter = 0;
            for _ in 0..b.get_pad_top() {
                if let Some(l) = b.get_left(iter) {
                    line.push(l);
                }
                line.push_str(&spacing);
                if let Some(r) = b.get_right(iter) {
                    line.push(r);
                }
                iter += 1;
                line.push('\n');
            }
            output.push_str(&line);
            return iter;
        }
        return 0;
    }

    fn justify_line(&self, ll: usize, mut width: usize) -> (usize, usize) {
        if let Some(b) = &self.border {
            width -= ll + b.get_pad_left() + b.get_pad_right() + b.width() + TOTAL_OFFSET;
        } else {
            width -= ll + TOTAL_OFFSET;
        }
        match self.justify {
            Justify::Left => (0, width),
            Justify::Right => (width, 0),
            Justify::Center => {
                if width % 2 != 0 {
                    (width / 2, width / 2 + 1)
                } else {
                    (width / 2, width / 2)
                }
            }
        }
    }

    fn construct_left_side(&self, line: &mut String, index: usize, iter: usize) {
        if let Some(b) = &self.border {
            // If Menu has left border
            if let Some(lb) = b.get_left(iter) {
                line.push(lb);
            }
            for _ in 0..(if index == self.cursor {
                b.get_pad_left()
            } else {
                b.get_pad_left() + CURSOR_OFFSET
            }) {
                line.push(' ');
            }
        }
    }

    fn construct_right_side(&self, line: &mut String, iter: usize) {
        if let Some(b) = &self.border {
            for _ in 0..b.get_pad_right() + CURSOR_OFFSET {
                line.push(' ');
            }
            if let Some(c) = b.get_right(iter) {
                line.push(c);
            }
        }
        line.push('\n');
    }

    fn render_item(&mut self, s: &mut String, width: usize, mut b_iter: usize) -> usize {
        for (i, item) in self.items.iter().enumerate() {
            let mut line = String::new();
            self.construct_left_side(&mut line, i, b_iter);
            let (just_left, just_right) = self.justify_line(item.label().len(), width);
            for _ in 0..just_left {
                line.push(' ');
            }
            if i == self.cursor {
                line.push_str(&self.marker.sprite().to_string());
                line.push(' ');
            }
            line.push_str(item.label());
            for _ in 0..just_right {
                line.push(' ');
            }
            self.construct_right_side(&mut line, b_iter);

            s.push_str(&line);
            b_iter += 1;
        }
        return b_iter;
    }

    fn largest_line(&self) -> usize {
        if self.items.len() == 0 {
            return 0;
        }
        let mut max = self.items[0].label().len();
        for each in self.items.iter() {
            if each.label().len() > max {
                max = each.label().len();
            }
        }
        return max;
    }

    fn update_max_per_page(&mut self, canvas: &Canvas) {
        let mut max: usize;
        if let Some(h) = &self.height {
            max = h.get(canvas.height - self.y);
            if let Some(b) = &self.border {
                max -= b.get_pad_top() + b.get_pad_bottom() + b.height();
            }
            self.max_per_page = max as u16;
        } else {
            max = canvas.height - self.y;
            if let Some(b) = &self.border {
                max -= b.get_pad_top() + b.get_pad_bottom() + b.height();
            }
            self.max_per_page = max as u16;
        }
    }
}

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
        fn output(&mut self) -> Output {
           Output::None 
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
