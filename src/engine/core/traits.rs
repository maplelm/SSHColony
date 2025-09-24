use std::sync::mpsc;
use std::hash::Hash;
use std::ops::{Add, Sub, Mul, Div};
use super::super::{input::Event, render::Msg};
use super::enums::Signal;
pub trait Scene<T: Scene<T>> {
    fn update( &mut self, delta_time: f32, event_rx: &mpsc::Receiver<Event>, render_tx: &mpsc::Sender<Msg>) -> Signal<T>;
    fn init(&mut self, render_tx: &mpsc::Sender<Msg>) -> Signal<T>;
    fn is_init(&self) -> bool;
    fn suspend(&mut self, render_tx: &mpsc::Sender<Msg>);
    fn resume(&mut self, render_tx: &mpsc::Sender<Msg>);
    fn is_paused(&self) -> bool;
    fn reset(&mut self);
}

pub trait Numeric: Copy 
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    {}

impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for f32 {}
impl Numeric for f64 {}
impl Numeric for usize {}
impl Numeric for isize {}

pub trait Storeable: for<'de> serde::Deserialize<'de> + serde::Serialize {
    type Key: serde::Serialize + for <'de> serde::Deserialize<'de> + Eq + Hash + Clone;
    fn key(&self) -> Self::Key;
}