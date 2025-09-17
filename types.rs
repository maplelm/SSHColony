use std::ops::{Add, Sub, Mul, Div};

pub struct Position<T> {
    x: T,
    y: T,
}

trait Numeric {
    Copy
    + PartialEq     // Allow ==
    + PartialOrd    // Allow < , >
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
}
