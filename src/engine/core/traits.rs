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
use std::ops::{Add, Div, Mul, Sub};
use std::sync::mpsc;

pub trait Scene {
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
        signal: Option<SceneDataMsg>,
        canvas: &Canvas,
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
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{}

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
