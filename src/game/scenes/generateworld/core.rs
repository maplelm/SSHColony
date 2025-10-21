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

use super::super::super::Game;
use crate::engine::{
    enums::{RenderSignal, Signal},
    input::Event,
    render::Canvas,
};
use std::sync::mpsc::{Receiver, Sender};

pub struct GenerateWorld {
    init_complete: bool,
}

impl GenerateWorld {
    pub fn init(&mut self, render_tx: &Sender<RenderSignal>, canvas: &Canvas) -> Signal<Game> {
        self.init_complete = true;
        Signal::None
    }

    pub fn is_init(&self) -> bool {
        self.init_complete
    }

    pub fn is_paused(&self) -> bool {
        false
    }

    pub fn reset(&mut self) {}

    pub fn resume(&mut self, redner_tx: &Sender<RenderSignal>, canvas: &Canvas) {}

    pub fn suspend(&mut self, render_tx: &Sender<RenderSignal>) {}

    pub fn update(
        &mut self,
        dt: f32,
        event: &Receiver<Event>,
        render_tx: &Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> Signal<Game> {
        Signal::None
    }
}
