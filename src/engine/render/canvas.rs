#[deny(unused)]

#[derive(Clone, Hash, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

