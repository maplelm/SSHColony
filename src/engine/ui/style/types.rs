use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Measure {
    Cell(u32),
    Percent(u8)
}

impl Measure {
    pub fn get(&self, max: usize) -> usize {
        match *self {
            Measure::Cell(val) => val as usize,
            Measure::Percent(val) => ((max as f64 / 100.0) * val as f64) as usize
        }
    }

    pub fn get_raw(&self) -> usize {
        match *self {
            Measure::Cell(val) => val as usize,
            Measure::Percent(val) => val as usize,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Origin {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Justify {
    Left,
    Right,
    Center
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Size {
    pub width: Option<Measure>,
    pub height: Option<Measure>,
}