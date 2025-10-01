use crate::engine::enums::RenderSignal;

use super::super::{input::Event, render::Canvas};
use super::enums::Signal;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::mpsc;
pub trait Scene<T: Scene<T>> {
    fn update(
        &mut self,
        delta_time: f32,
        event_rx: &mpsc::Receiver<Event>,
        render_tx: &mpsc::Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> Signal<T>;
    fn init(&mut self, render_tx: &mpsc::Sender<RenderSignal>, canvas: &Canvas) -> Signal<T>;
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
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
}

impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for f32 {}
impl Numeric for f64 {}
impl Numeric for usize {}
impl Numeric for isize {}

pub trait Storeable: for<'de> serde::Deserialize<'de> + serde::Serialize {
    type Key: serde::Serialize + for<'de> serde::Deserialize<'de> + Eq + Hash + Clone;
    fn key(&self) -> Self::Key;
}
