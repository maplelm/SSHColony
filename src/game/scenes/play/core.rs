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

use crate::game::Game;
use crate::engine::{
    render::Canvas,
    enums::{RenderSignal, Signal as EngineSignal},
    input::Event as InputEvent
};
use std::sync::mpsc;

enum Signals {
    Exit,
    Save,
    Pause,
}

pub struct PlayGame {
    init_complete: bool,
    paused: bool
}

impl PlayGame {
    pub fn new() -> Game {
        Game::PlayGame(Self{
            init_complete: false,
            paused: false
        })
    }

    pub fn init(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
        _canvas: &Canvas,
    ) -> EngineSignal<Game> {
        EngineSignal::None
    }

    pub fn is_init(&self) -> bool {
        self.init_complete
    }

    pub fn update(
        &mut self,
        _delta_time: f32,
        event: &mpsc::Receiver<InputEvent>,
        render_tx: &mpsc::Sender<RenderSignal>,
        _canvas: &Canvas,
    ) -> EngineSignal<Game> {
        EngineSignal::None
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn reset(&mut self) {}

    pub fn resume(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
        _canvas: &Canvas,
    ) {}

    pub fn suspend(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
    ) {}
}