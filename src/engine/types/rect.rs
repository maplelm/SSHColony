use crate::engine::traits::Numeric;

pub struct Rect<T: Numeric> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T: Numeric> Rect<T> {
    pub fn area(&self) -> T {
        self.w * self.h
    }
}

pub struct Rect3D<T: Numeric> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    pub h: T,
    pub d: T,
}

impl<T: Numeric> Rect3D<T> {
    pub fn volume(&self) -> T {
        self.w * self.h * self.d
    }
}
