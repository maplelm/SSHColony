use super::super::enums::RenderSignal;
use super::Border;
use crate::engine::types::Position;
use crate::engine::{
    render::{Canvas, Layer, Object, RenderUnitId},
    ui::style::Measure,
};
use std::sync::{Arc, Weak, mpsc::Sender};
use term::color::{Background, Foreground};

pub struct SelectorItem {
    pub label: String,
    pub value: usize,
}

pub enum SelectionDirection {
    Vertical,
    Horizontal,
}

pub struct Selector {
    pub render_id: Weak<RenderUnitId>,
    pos: Position<i32>,
    width: Option<Measure>,
    height: Option<Measure>,
    items: Vec<SelectorItem>,
    select_foreground: Option<Foreground>,
    select_background: Option<Background>,
    hover_foreground: Option<Foreground>,
    hover_background: Option<Background>,
    foreground: Option<Foreground>,
    background: Option<Background>,
    direction: SelectionDirection,
    border: Option<Border>,
    selected: Option<usize>,
    cursor: usize,
}

impl Selector {
    pub fn new(
        x: i32,
        y: i32,
        width: Option<Measure>,
        height: Option<Measure>,
        select_fg: Option<Foreground>,
        select_bg: Option<Background>,
        hover_fg: Option<Foreground>,
        hover_bg: Option<Background>,
        fg: Option<Foreground>,
        bg: Option<Background>,
        direction: SelectionDirection,
        border: Option<Border>,
        items: Vec<SelectorItem>,
    ) -> Self {
        Self {
            render_id: Weak::new(),
            pos: Position { x: x, y: y },
            width: width,
            height: height,
            items: items,
            border: border,
            direction: direction,
            select_foreground: select_fg,
            select_background: select_bg,
            hover_foreground: hover_fg,
            hover_background: hover_bg,
            foreground: fg,
            background: bg,
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

    fn contruct_line_color(
        s: &str,
        output: &mut String,
        fg: Option<Foreground>,
        bg: Option<Background>,
        sfg: Option<Foreground>,
        sbg: Option<Background>,
    ) {
        if let Some(fg) = sfg {
            output.push_str(&fg.to_ansi());
        }
        if let Some(bg) = sbg {
            output.push_str(&bg.to_ansi());
        }
        output.push_str(s);
        if sbg.is_some() || sfg.is_some() {
            output.push_str("\x1b[0m");
            if let Some(fg) = fg {
                output.push_str(&fg.to_ansi());
            }
            if let Some(bg) = bg {
                output.push_str(&bg.to_ansi());
            }
        }
    }

    pub fn output(&mut self, render_tx: &Sender<RenderSignal>) {
        let mut output: String = String::new();
        let output = match self.direction {
            SelectionDirection::Vertical => {
                for (i, mut each) in self.items.iter().enumerate() {
                    if i == self.cursor {
                        Selector::contruct_line_color(
                            each.label.as_str(),
                            &mut output,
                            self.foreground,
                            self.background,
                            self.select_foreground,
                            self.select_background,
                        );
                    } else {
                        output.push_str(&each.label);
                    }
                    if i < self.items.len() - 1 {
                        output.push_str("  ");
                    }
                }
                output
            }
            SelectionDirection::Horizontal => {
                for (i, each) in self.items.iter().enumerate() {
                    if i == self.cursor {
                        Selector::contruct_line_color(
                            each.label.as_str(),
                            &mut output,
                            self.foreground,
                            self.background,
                            self.select_foreground,
                            self.select_background,
                        );
                    } else {
                        output.push_str(&each.label);
                    }
                    if i < self.items.len() - 1 {
                        output.push_str("\n\n");
                    }
                }
                output
            }
        };

        match self.render_id.upgrade() {
            None => {
                let arc_id = RenderUnitId::new(Layer::Ui);
                self.render_id = Arc::downgrade(&arc_id);
                if let Err(_e) = render_tx.send(RenderSignal::Insert(
                    arc_id,
                    Object::static_text(
                        self.pos.as_3d(0),
                        output,
                        super::style::Justify::Center,
                        super::style::Align::Center,
                        self.width,
                        self.height,
                        self.border.clone(),
                        self.foreground,
                        self.background,
                    ),
                )) {
                    // Log that there was a problem
                }
            }
            Some(arc) => {
                if let Err(_e) = render_tx.send(RenderSignal::Update(
                    arc,
                    Object::static_text(
                        self.pos.as_3d(0),
                        output,
                        super::style::Justify::Center,
                        super::style::Align::Center,
                        self.width,
                        self.height,
                        self.border.clone(),
                        self.foreground,
                        self.background,
                    ),
                )) {
                    // Log that there was a problem
                }
            }
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
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            SelectionDirection::Horizontal,
            None,
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
