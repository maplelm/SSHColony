use std::io::{Read, Write, stdin};

use super::{Action, Border};
use crate::app::{self, BindGroup, KeyBuf, KeyMap, CLEAR_BUFFER};
use crate::traits::{self, Renderable, UiElement};

pub struct Item<S: traits::Scene> {
    pub label: String,
    pub action: Action<S>,
}

pub struct Menu<S: traits::Scene> {
    x: u32,
    y: u32,
    pub border: Option<Border>,
    pub label_style: Option<String>,
    pub marker_style: Option<String>,
    
    pub bindings: crate::app::input::BindGroup<S>,
    marker: String,
    items: Vec<Item<S>>,
    cursor: usize,
    h_border_cache: String,
}

impl<S: traits::Scene> Menu<S> {
    pub fn new(x: u32, y: u32, border: Option<Border>, items: Option<Vec<Item<S>>>) -> Self {
        let mut obj = Self {
            x: x,
            y: y,
            items: if let Some(items) = items {
                items
            } else {
                vec![]
            },
            bindings: BindGroup::new(),
            border: border,
            //marker: String::from("\x1b[32m\x1b[42m \x1b[0m"),
            label_style: None,
            marker_style: None,
            marker: String::from(">"),
            cursor: 0,
            h_border_cache: String::new(),
        };
        obj.build_horizontal_border();
        return obj;
    }

    fn get_width(&self) -> Option<usize> {
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
            None => Some(w + self.marker.len()),
            Some(b) => Some(
                w + self.marker.len()
                    + b.left_padding as usize
                    + b.right_padding as usize
                    + b.left.len()
                    + b.right.len(),
            ),
        }
    }

    fn build_horizontal_border(&mut self) {
        if self.border.is_none() {
            // Nothing to do, there is no border
            return;
        }
        let b = self.border.as_ref().unwrap();
        let len = (self.marker.len() as isize + self.get_width().unwrap_or(0) as isize)
            - (b.right.len() as isize + b.left.len() as isize);
        if len <= 0 {
            return;
        }
        while self.h_border_cache.len() < len as usize {
            self.h_border_cache.push_str(&b.top)
        }
        while self.h_border_cache.len() > len as usize {
            self.h_border_cache.pop();
        }
    }

    pub fn add(&mut self, item: Item<S>) {
        self.items.push(item);
        self.build_horizontal_border();
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

    pub fn action(&self) -> Action<S> {
        self.items[self.cursor].action
    }
}

impl<S: traits::Scene> Renderable for Menu<S> {
    fn render(&self, ctx: &mut app::core::Context) -> Result<(), std::io::Error> {
        let x = self.x;
        let y = self.y;
        let m = &self.marker;
        let m_blank = String::from(" ").repeat(m.len());
        match self.border.as_ref() {
            None => {
                for i in 0..self.items.len() {
                    let label = &self.items[i].label;
                    let y: u32 = y + i as u32;
                    if i == self.cursor {
                        if let Err(e) = write!(ctx.stdout_buffer, "\x1b{y};{x}f{m}{label}") {
                            return Err(e);
                        }
                    } else {
                        if let Err(e) = write!(ctx.stdout_buffer, "\x1b[{y};{x}f{m_blank}{label}") {
                            return Err(e);
                        }
                    }
                    if let Err(e) = ctx.stdout_buffer.flush() {
                        return Err(e);
                    }
                }
                Ok(())
            }
            Some(b) => {
                let l = &b.left;
                let c = &self.h_border_cache;
                let r = &b.right;
                let lp = b.left_padding;
                let rp = b.right_padding;
                let label_len =
                    self.h_border_cache.len() - (b.left_padding + b.right_padding) as usize;
                let m = &self.marker;
                let m_blank = String::from(" ").repeat(m.len());
                // Render Top Border
                if let Err(e) = write!(ctx.stdout_buffer, "\x1b[{y};{x}f{l}{c}{r}") {
                    return Err(e);
                }
                if let Err(e) = ctx.stdout_buffer.flush() {
                    return Err(e);
                }
                // Render Line Items
                for i in 0..self.items.len() {
                    let label = &self.items[i].label;
                    let y: u32 = y + i as u32 + 1;
                    let rp = rp + (label_len - label.len()) as i16;
                    if i == self.cursor {
                        if let Err(e) = write!(
                            ctx.stdout_buffer,
                            "\x1b[{y};{x}f{l}\x1b[{lp}C{m}{label}\x1b[{rp}C{r}"
                        ) {
                            return Err(e);
                        }
                    } else {
                        if let Err(e) = write!(
                            ctx.stdout_buffer,
                            "\x1b[{y};{x}f{l}\x1b[{lp}C{m_blank}{label}\x1b[{rp}C{r}"
                        ) {
                            return Err(e);
                        }
                    }
                    if let Err(e) = ctx.stdout_buffer.flush() {
                        return Err(e);
                    }
                }
                // Render Bottom Line
                let y = y + self.items.len() as u32 + 1;
                if let Err(e) = write!(ctx.stdout_buffer, "\x1b[{y};{x}f{l}{c}{r}") {
                    return Err(e);
                }
                if let Err(e) = ctx.stdout_buffer.flush() {
                    return Err(e);
                }
                Ok(())
            }
        }
    }
}

impl<S: traits::Scene> UiElement for Menu<S> {
    fn update(&mut self) {}
}

