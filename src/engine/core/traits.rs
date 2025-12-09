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
use crate::engine::enums::RenderSignal;

use super::super::{input::Event, render::Canvas};
use super::enums::Signal;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::sync::{Arc, mpsc};

pub trait Scene: std::fmt::Debug {
    fn update(
        &mut self,
        delta_time: f32,
        event_rx: &mpsc::Receiver<Event>,
        render_tx: &mpsc::Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> Signal;
    fn init(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
        signal: Option<Signal>,
        canvas: &Canvas,
        lg: Arc<logging::Logger>,
    ) -> Signal;
    fn is_init(&self) -> bool;
    fn suspend(&mut self, render_tx: &mpsc::Sender<RenderSignal>);
    fn resume(&mut self, render_tx: &mpsc::Sender<RenderSignal>, canvas: &Canvas);
    fn is_paused(&self) -> bool;
    fn reset(&mut self);
}

pub trait Numeric:
    Copy
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
{
    fn zero() -> Self;
    fn as_i32(&self) -> i32;
}

impl Numeric for u8 {
    fn zero() -> Self {
        0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl Numeric for u16 {
    fn zero() -> Self {
        0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl Numeric for u32 {
    fn zero() -> Self {
        0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl Numeric for u64 {
    fn zero() -> Self {
        0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl Numeric for i8 {
    fn zero() -> Self {
        0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl Numeric for i16 {
    fn zero() -> Self {
        0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl Numeric for i32 {
    fn zero() -> Self {
        0
    }
    fn as_i32(&self) -> i32 {
        *self
    }
}
impl Numeric for i64 {
    fn zero() -> Self {
        0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl Numeric for f32 {
    fn zero() -> Self {
        0.0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl Numeric for f64 {
    fn zero() -> Self {
        0.0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl Numeric for usize {
    fn zero() -> Self {
        0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
impl Numeric for isize {
    fn zero() -> Self {
        0
    }
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}

pub trait Storeable: for<'de> serde::Deserialize<'de> + serde::Serialize {
    type Key: serde::Serialize + for<'de> serde::Deserialize<'de> + Eq + Hash + Clone;
    fn key(&self) -> Self::Key;
}
