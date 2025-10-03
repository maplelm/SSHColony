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
    pub fn as_3d(&self, depth: T) -> Position3D<T> {
        Position3D { x: self.x, y: self.y, z: depth }
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

    pub fn as_2d(&self) -> Position<T> {
        Position { x: self.x, y: self.y }
    }
}