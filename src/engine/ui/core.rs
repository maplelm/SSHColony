use super::style::{Measure, Origin};

pub trait UIElement<R> {
    fn update(&mut self) -> Option<R>;
    fn output(&self) -> String;
}

pub struct DisplayProperties {
    pub x: usize,
    pub y: usize,
    pub w: Option<Measure>,
    pub h: Option<Measure>,
    pub o: Origin,
}

impl DisplayProperties {
    pub fn new(x: usize, y: usize, w: Option<Measure>, h: Option<Measure>, o: Origin) -> Self {
        Self {
            x: x,
            y: y,
            w: w,
            h: h,
            o: o,
        }
    }
}

impl Default for DisplayProperties {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            w: None,
            h: None,
            o: Origin::TopLeft,
        }
    }
}
