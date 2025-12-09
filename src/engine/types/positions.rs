/*
Copyright 2025 Luke Maple

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
you may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::super::traits::Numeric;

#[derive(
    Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub struct Position<T: Numeric> {
    pub x: T,
    pub y: T,
}

impl Into<Position<i32>> for Position<usize> {
    fn into(self) -> Position<i32> {
        Position {
            x: self.x.as_i32(),
            y: self.y.as_i32(),
        }
    }
}

impl<T: Numeric> Into<Position3D<T>> for Position<T> {
    fn into(self) -> Position3D<T> {
        Position3D {
            x: self.x,
            y: self.y,
            z: T::zero(),
        }
    }
}

impl<T: Numeric> Default for Position<T> {
    fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T: Numeric> Add for Position<T> {
    type Output = Position<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Numeric> AddAssign for Position<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Numeric> Sub for Position<T> {
    type Output = Position<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Numeric> SubAssign for Position<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Numeric> Mul for Position<T> {
    type Output = Position<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Numeric> MulAssign for Position<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Numeric> Div for Position<T> {
    type Output = Position<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Numeric> DivAssign for Position<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: Numeric> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }

    pub fn add(self, x: T, y: T) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn join(self, pos: Position<T>) -> Self {
        Self {
            x: self.x + pos.x,
            y: self.y + pos.y,
        }
    }
}

#[derive(
    Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub struct Position3D<T: Numeric> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Into<Position3D<i32>> for Position3D<usize> {
    fn into(self) -> Position3D<i32> {
        Position3D {
            x: self.x as i32,
            y: self.y as i32,
            z: self.z as i32,
        }
    }
}

impl<T: Numeric> Add for Position3D<T> {
    type Output = Position3D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Numeric> Sub for Position3D<T> {
    type Output = Position3D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Numeric> Mul for Position3D<T> {
    type Output = Position3D<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Numeric> Div for Position3D<T> {
    type Output = Position3D<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Numeric> AddAssign for Position3D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Numeric> SubAssign for Position3D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x - rhs.x;
        self.y - rhs.y;
        self.z - rhs.z;
    }
}

impl<T: Numeric> MulAssign for Position3D<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x * rhs.x;
        self.y * rhs.y;
        self.z * rhs.z;
    }
}

impl<T: Numeric> DivAssign for Position3D<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x / rhs.x;
        self.y / rhs.y;
        self.z / rhs.z;
    }
}

impl<T: Numeric> Into<Position<T>> for Position3D<T> {
    fn into(self) -> Position<T> {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

impl<T: Numeric> Default for Position3D<T> {
    fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

impl<T: Numeric> Position3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn add(mut self, x: T, y: T, z: T) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }

    pub fn join(mut self, pos: Position3D<T>) -> Self {
        Self {
            x: self.x + pos.x,
            y: self.y + pos.y,
            z: self.z + pos.z,
        }
    }
}
