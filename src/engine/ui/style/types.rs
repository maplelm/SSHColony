use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Measure {
    Cell(u32),
    Percent(u8),
}

impl Measure {
    pub fn get(&self, max: usize) -> usize {
        match *self {
            Measure::Cell(val) => val as usize,
            Measure::Percent(val) => ((max as f64 / 100.0) * val as f64) as usize,
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
    Center,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Justify {
    Left,
    Right,
    Center,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Align {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Size {
    pub width: Option<Measure>,
    pub height: Option<Measure>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::engine::consts::DEFAULT_CANVAS;
    use term::term_size;
    #[test]
    fn measure_test() {
        let mut canvas = DEFAULT_CANVAS;
        if let Some(size) = term_size() {
            canvas.width = size.0 as usize;
            canvas.height = size.1 as usize;
        }

        let x = Measure::Cell(40);
        assert_eq!(x.get(canvas.height), 40);
        let x = Measure::Cell(60);
        assert_eq!(x.get(canvas.height), 60);
        let x = Measure::Percent(100);
        assert_eq!(x.get(canvas.height), canvas.height);
        let x = Measure::Percent(50);
        assert_eq!(x.get(canvas.height), canvas.height / 2);
        let x = Measure::Percent(25);
        assert_eq!(x.get(canvas.height), canvas.height / 4);
    }
}
