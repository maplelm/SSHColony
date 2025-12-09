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

use crate::{
    engine::{
        self, Error,
        enums::{RenderSignal, Signal},
        input::{Event, KeyEvent},
        render::{self, Canvas},
        traits::Scene,
    },
    game::types::World,
};
use std::{
    marker::PhantomData,
    sync::{Arc, mpsc},
};

#[derive(Debug)]
pub struct InGame {
    world: World,
    init_complete: bool,
    is_paused: bool,
}

const DEFAULT_WORLD_X: usize = 50;
const DEFAULT_WORLD_Y: usize = 50;
const DEFAULT_WORLD_Z: usize = 1;
const DEFAULT_WORLD_TEMP: f32 = 25.0; // Celius
const DEFAULT_WORLD_HEIGHT: f32 = 1.0;
const DEFAULT_WORLD_SEA_LEVEL: f32 = 0.0;

impl InGame {
    pub fn new() -> Result<Box<dyn Scene>, Error> {
        let mut world: World;
        match World::new(
            "test_world".to_string(),
            DEFAULT_WORLD_X,
            DEFAULT_WORLD_Y,
            DEFAULT_WORLD_Z,
            DEFAULT_WORLD_TEMP,
            DEFAULT_WORLD_HEIGHT,
            DEFAULT_WORLD_SEA_LEVEL,
            "./data/materials/",
            "./data/entities/",
            "./data/sprites/",
        ) {
            Err(e) => return Err(e),
            Ok(w) => world = w,
        }
        Ok(Box::new(Self {
            world: world,
            init_complete: false,
            is_paused: false,
        }))
    }
}

impl Scene for InGame {
    fn init(
        &mut self,
        _render_tx: &mpsc::Sender<RenderSignal>,
        signal: Option<Signal>,
        canvas: &Canvas,
        lg: Arc<logging::Logger>,
    ) -> Signal {
        let _ = self.world.generate(None);
        self.init_complete = true;

        Signal::None
    }
    fn is_init(&self) -> bool {
        self.init_complete
    }
    fn is_paused(&self) -> bool {
        self.is_paused
    }
    fn reset(&mut self) {}
    fn resume(&mut self, render_tx: &mpsc::Sender<RenderSignal>, canvas: &Canvas) {}
    fn suspend(&mut self, render_tx: &mpsc::Sender<RenderSignal>) {}
    fn update(
        &mut self,
        delta_time: f32,
        event: &mpsc::Receiver<Event>,
        render_tx: &std::sync::mpsc::Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> Signal {
        for event in event.try_iter() {
            match event {
                Event::Keyboard(key) => match key {
                    KeyEvent::Char('q') => return Signal::Quit,
                    _ => {}
                },
                _ => {}
            }
        }
        Signal::None
    }
}
