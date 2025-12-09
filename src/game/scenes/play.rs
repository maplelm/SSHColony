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

use crate::engine::{
    enums::{RenderSignal, Signal as EngineSignal},
    input::Event as InputEvent,
    render::Canvas,
    traits::Scene,
};
use std::sync::{Arc, mpsc};

#[derive(Debug)]
enum Signals {
    Exit,
    Save,
    Pause,
}

#[derive(Debug)]
pub struct PlayGame {
    init_complete: bool,
    paused: bool,
}
impl PlayGame {
    pub fn new() -> Box<dyn Scene> {
        Box::new(Self {
            init_complete: false,
            paused: false,
        })
    }
}

impl Scene for PlayGame {
    fn init(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
        signal: Option<EngineSignal>,
        _canvas: &Canvas,
        lg: Arc<logging::Logger>,
    ) -> EngineSignal {
        EngineSignal::None
    }

    fn is_init(&self) -> bool {
        self.init_complete
    }

    fn update(
        &mut self,
        _delta_time: f32,
        event: &mpsc::Receiver<InputEvent>,
        render_tx: &mpsc::Sender<RenderSignal>,
        _canvas: &Canvas,
    ) -> EngineSignal {
        EngineSignal::None
    }

    fn is_paused(&self) -> bool {
        self.paused
    }

    fn reset(&mut self) {}

    fn resume(&mut self, render_tx: &mpsc::Sender<RenderSignal>, _canvas: &Canvas) {}

    fn suspend(&mut self, render_tx: &mpsc::Sender<RenderSignal>) {}
}
