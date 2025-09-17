use super::{Border, UIElement};
use crate::engine::render;
use crate::engine::types::Position;
use crate::engine::Scene;

pub struct Item<I, O> {
    pub label: String,
    pub action: fn(&I) -> Option<O>,
}

pub struct Menu<I, O> {
    x: usize,
    y: usize,
    pub border: Option<Border<char>>,
    pub label_style: Option<String>,
    pub marker_style: Option<String>,
    pub marker: char,

    items: Vec<Item<I, O>>,
    cursor: usize,
}

impl<I, O> Menu<I, O> {
    pub fn new(
        x: usize,
        y: usize,
        border: Option<Border<char>>,
        items: Vec<Item<I, O>>,
    ) -> Self {
        Self {
            x: x,
            y: y,
            items: items,
            border: border,
            label_style: None,
            marker_style: None,
            marker: '>',
            cursor: 0,
        }
    }

    pub fn x(&self) -> usize { self.x}
    pub fn y(&self) -> usize { self.y}

    pub fn execute(&self, s: &I) -> Option<O> {
        (self.items[self.cursor].action)(s)
    }

    pub fn cursor_pos(&self) -> Position<usize> {
        match &self.border {
            None => Position::new(self.x, self.y + self.cursor),
            
            Some(b) => Position::new(self.x + 1 + b.padding.left, self.y + 1 + b.padding.top + self.cursor)
        }
    }

    pub fn marker_object(&self) -> Option<render::Object> {
        render::Object::new_static(String::from(self.marker))
    }

    pub fn get_width(&self) -> Option<usize> {
        if self.items.len() == 0 {
            return None;
        }
        let mut w = 0;
        for each in self.items.iter() {
            if each.label.len() > w {
                w = each.label.len()
            }
        }
        match &self.border {
            None => Some(
                w + // With of largest item
                2 // Marker plus space
            ),
            Some(b) => Some(
                    w +  // Width of larget item
                    2 +  // Marker plus space
                    b.padding.left as usize +
                    b.padding.right as usize + 
                    2 // the actual left and right border
            ),
        }
    }

    pub fn add(&mut self, item: Item<I, O>) {
        self.items.push(item);
    }

    pub fn cursor_up(&mut self, amount: usize) {
        if self.cursor as isize - amount as isize >= 0 {
            self.cursor -= amount;
        }
    }

    pub fn cursor_down(&mut self, amount: usize) {
        if self.cursor + amount < self.items.len() {
            self.cursor += amount;
        }
    }
}

impl<I, O> UIElement<O> for Menu<I, O> {
    fn update(&mut self) -> Option<O> {
        None
    }
    fn output(&self) -> String {
        match &self.border {
            None => {return String::new();}
            Some(b) => {
                let mut out = String::new();
                if self.items.len() == 0 {
                    return out;
                }
                // Top Border (##########)
                let mut h_border = String::new();
                for _ in 0..self.get_width().unwrap(){
                    h_border.push(b.top);
                }
                out.push_str(&h_border);
                out.push('\n');
                // Top Padding lines ( #         #)
                if b.padding.top > 0 {
                    let mut h_padding = String::new();
                    h_padding.push(b.left);
                    for _ in 0 .. self.get_width().unwrap()-2 {
                        h_padding.push(' ');
                    }
                    h_padding.push(b.right);
                    for _ in 0..b.padding.top {
                        out.push_str(&h_padding);
                        out.push('\n');
                    }
                }
                // Item Lines ( #  item 1  #) ( # > item 2 #
                for (i, item) in self.items.iter().enumerate() {
                    let mut line = String::new();
                    line.push(b.left);
                    if i == self.cursor {
                        for _ in 0..b.padding.left {
                            line.push(' ');
                        }
                        line.push(self.marker);
                        line.push(' ');
                        
                    } else {
                        for _ in 0..b.padding.left + 2 {
                            line.push(' ');
                        }
                    }
                    line.push_str(&item.label);
                    for _ in 0..b.padding.right {
                        line.push(' ');
                    }
                    while line.len() < self.get_width().unwrap_or(0) -1 {
                        line.push(' ');
                    }
                    line.push(b.right);
                    line.push('\n');
                    out.push_str(&line);
                }
                // Bottom Padding lines ( #         #)
                if b.padding.bottom > 0 {
                    let mut h_padding = String::new();
                    h_padding.push(b.left);
                    for _ in 0 .. self.get_width().unwrap()-2 {
                        h_padding.push(' ');
                    }
                    h_padding.push(b.right);
                    for _ in 0..b.padding.top {
                        out.push_str(&h_padding);
                        out.push('\n');
                    }
                }
                // Bottom Border (###########)
                out.push_str(&h_border);
                return out;
            }
        }
    }
}
