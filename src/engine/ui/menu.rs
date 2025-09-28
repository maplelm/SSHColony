#![deny(unused)]
use crate::engine::render::Object;
use crate::engine::ui::style::Justify;

use super::super::render::Canvas;
use super::{
    super::{render, types::*},
    Border, DisplayProperties,
    style::{Measure, Origin},
};

const CURSOR_OFFSET: usize = 2;
const TOTAL_OFFSET: usize = CURSOR_OFFSET * 2;

// -------------
// Name: Item
// Usage: Added to Menus for each option the menu should have
// -------------
pub struct Item<I, O> {
    pub label: String,
    pub action: fn(I) -> Option<O>,
}

impl<I, O> Item<I, O> {
    pub fn new(l: String, a: fn(I) -> Option<O>) -> Self {
        Self {
            label: l,
            action: a,
        }
    }
}

//////////////
///  MENU  ///
//////////////
pub struct Menu<I, O> {
    display_properties: DisplayProperties,
    pub border: Option<Border>,
    pub _label_style: Option<String>,
    pub marker: Object,
    pub justify: Justify,

    items: Vec<Item<I, O>>,
    cursor: usize,
    max_per_page: u16,
    _page: u16,
}

impl<I, O> Menu<I, O> {
    pub fn new(
        x: usize,
        y: usize,
        w: Option<Measure>,
        h: Option<Measure>,
        origin: Origin,
        justify: Justify,
        border: Option<Border>,
        items: Vec<Item<I, O>>,
    ) -> Self {
        Self {
            display_properties: DisplayProperties::new(x, y, w, h, origin),
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
        self.display_properties.x
    }
    pub fn y(&self) -> usize {
        self.display_properties.y
    }
    pub fn width(&self, canvas: &Canvas) -> usize {
        match &self.display_properties.w {
            // Specified width
            Some(w) => match *w {
                Measure::Cell(w) => w as usize,
                Measure::Percent(w) => ((canvas.width as f64 / 100.0) * w as f64) as usize,
            },
            // Calculate width based on length of line items
            None => {
                let mut output = 0;
                output += find_largest_line(&self);
                if let Some(b) = &self.border {
                    output += b.get_pad_left() + b.get_pad_right() + b.width();
                }
                output += TOTAL_OFFSET;
                return output;
            }
        }
    }

    pub fn execute(&self, s: I) -> Option<O> {
        (self.items[self.cursor].action)(s)
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

    // TODO: have menu output handle borderless menus
    pub fn output(&mut self, canvas: &Canvas) -> String {
        let mut out = String::new();
        if self.items.len() == 0 {
            return out;
        }
        let mut border_iter: usize = 0;
        self.max_per_page = calculate_max_per_page(self, canvas);
        let width = self.width(canvas);
        if let Some(b) = self.border.as_mut() {
            top_border_render(b, &mut out, width);
            border_iter = top_padding_render(b, &mut out, width, border_iter);
        }
        border_iter = line_item_render(self, &mut out, width, border_iter);
        if let Some(b) = self.border.as_mut() {
            bottom_padding_render(b, &mut out, width, border_iter);
            bottom_border_render(b, &mut out, width);
        }
        if self.border.is_some() {
        }
        return out;
    }
}

fn top_padding_render(b: &mut Border, s: &mut String, width: usize, mut b_iter: usize) -> usize {
    if b.get_pad_top() == 0 {
        return b_iter;
    }
    for _ in 0..b.get_pad_top() {
        let mut h_padding = String::new();
        h_padding.push_str(& if let Some(b) = b.get_left(b_iter) {
            b.to_string()
        } else {
            "".to_string()
        });
        for _ in 0..width - b.width() {
            h_padding.push(' ');
        }
        h_padding.push_str(&if let Some(b) = b.get_right(b_iter){
            b.to_string()
        } else {
            "".to_string()
        });
        h_padding.push('\n');
        s.push_str(&h_padding);
        b_iter += 1;
    }
    b_iter
}

fn top_border_render(b: &mut Border, s: &mut String, width: usize) {
    // Top Border
    let mut b_iter: usize = 0;
    let mut h_border = String::new();
    while let Some(c) = b.get_top(b_iter) && h_border.len() < width{
        h_border.push(c);
        b_iter += 1;
    }
    s.push_str(&h_border);
    s.push('\n');
}

fn line_item_render<I, O>(menu: &mut Menu<I, O>, s: &mut String, width: usize, mut b_iter: usize) -> usize {
    for (i, item) in menu.items.iter().enumerate() {
        let mut line = String::new();
        line_border_padding_offset_left(&mut line, &menu.border, i == menu.cursor, b_iter);
        if i == menu.cursor {
            line.push_str(&menu.marker.sprite().to_string());
            line.push(' ');
        }
        let (lspace, rspace) = line_space_calc(menu, item.label.len(), width);

        for _ in 0..lspace {
            line.push(' ');
        }
        line.push_str(&item.label);
        for _ in 0..rspace {
            line.push(' ');
        }

        line_border_padding_offset_rigth(&mut line, &menu.border, b_iter);

        line.push('\n');
        s.push_str(&line);
        b_iter += 1;
    }
    b_iter
}

fn bottom_padding_render(b: &mut Border, s: &mut String, width: usize, mut b_iter: usize) -> usize {
    if b.get_pad_bottom() == 0 {
        return b_iter;
    }
    for _ in 0..b.get_pad_bottom() {
        let mut h_padding = 
            if let Some(b) = b.get_left(b_iter){
                b.to_string()
            } else {
                "".to_string()
        };
        for _ in 0..width - b.width() {
            h_padding.push(' ');
        }
        h_padding.push_str(
            &if let Some(b) = b.get_right(b_iter) {
                b.to_string()
            } else {
                "".to_string()
            }
        );
        h_padding.push('\n');
        s.push_str(&h_padding);
        b_iter += 1;
    }
    b_iter
}

fn bottom_border_render(b: &mut Border, s: &mut String, width: usize) {
    let mut b_iter: usize = 0;
    for _ in 0..width {
        if let Some(c) = b.get_bottom(b_iter) {
            s.push(c);
        }
        b_iter += 1;
    }
}

fn find_largest_line<I, O>(menu: &Menu<I, O>) -> usize {
    if menu.items.len() < 1 {
        return 0;
    }
    let mut max = menu.items[0].label.len();
    for line in menu.items.iter() {
        if line.label.len() > max {
            max = line.label.len();
        }
    }
    max
}

fn calculate_max_per_page<I, O>(menu: &mut Menu<I, O>, canvas: &Canvas) -> u16 {
    let mut max: usize;
    if let Some(h) = &menu.display_properties.h {
        max = h.get(canvas.height);
        if let Some(b) = &menu.border {
            max -= b.get_pad_top() + b.get_pad_bottom() + b.height();
        }
    } else {
        max = canvas.height;
        if let Some(b) = &menu.border {
            max -= b.get_pad_top() + b.get_pad_bottom() + b.height();
        }
    }
    return max as u16;
}

fn line_border_padding_offset_rigth(line: &mut String, b: &Option<Border>, b_iter: usize) {
        if let Some(b) = b {
            for _ in 0..b.get_pad_right() + CURSOR_OFFSET {
                line.push(' ');
            }
            if let Some(c) = b.get_right(b_iter) {
                line.push(c);
            }
        }
}

fn line_border_padding_offset_left(line: &mut String, b: &Option<Border>, is_cursor_line: bool, b_iter: usize) {
    if let Some(b) = b {
        if let Some(c) = b.get_left(b_iter) {
            line.push(c);
        }
        if is_cursor_line {
            for _ in 0..b.get_pad_left() {
                line.push(' ');
            }
        } else {
            for _ in 0..b.get_pad_left() + CURSOR_OFFSET {
                line.push(' ');
            }
        }
    }
}

fn line_space_calc<I, O>(menu: &Menu<I, O>, label_len: usize, width: usize) -> (usize, usize) {
    let mut extra = width;
    if let Some(b) = &menu.border {
        extra -= label_len + b.get_pad_left() + b.get_pad_right() + b.width() + TOTAL_OFFSET;
    } else {
        extra -= label_len + TOTAL_OFFSET;
    }
    match menu.justify {
        Justify::Left => (0, extra),
        Justify::Right => (extra, 0),
        Justify::Center => {
            if extra % 2 == 0 {
                return (extra / 2, extra / 2);
            }
            return (extra / 2, extra / 2 + 1);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::engine::ui::{BorderSprite, Padding};

    use super::*;

    #[test]
    fn dynamic_length_menu() {
        let s1 = String::from("testing");
        let s2 = String::from("testing again");
        let s3 = String::from("testing and again");
        let c = Canvas::new(100, 7);
        let mut m = Menu::<(), ()>::new(
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
                Item::new(s1, |_| None),
                Item::new(s2, |_| None),
                Item::new(s3.clone(), |_| None),
            ],
        );
        let output = m.output(&c);
        println!("{}", output);
        for each in output.split('\n') {
            assert_eq!(each.len(), s3.len() + 8);
        }
    }

    #[test]
    fn fixed_length_menu() {
        let c = Canvas::new(40, 7);
        let mut m = Menu::<(), ()>::new(
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
                Item::new("testing".to_string(), |_| None),
                Item::new("testing again".to_string(), |_| None),
                Item::new("testing and again".to_string(), |_| None),
            ],
        );
        let out = m.output(&c);
        println!("{}", out);
        for each in out.split('\n') {
            assert_eq!(each.len(), c.width);
        }
    }
}
