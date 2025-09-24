use super::super::traits::Numeric;

#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub struct Position<T: Numeric> {
    pub x: T,
    pub y: T,
}

impl<T: Numeric> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub struct Position3D<T: Numeric> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Numeric> Position3D<T> {
    pub fn new(x: T, y: T, z: T ) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }
}