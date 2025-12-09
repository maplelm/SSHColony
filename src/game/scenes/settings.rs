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

#![allow(unused)]
use crate::engine::{
    self,
    enums::{RenderSignal, SceneSignal, Signal},
    input::{Event, KeyEvent},
    render::{self, Canvas, Object, RenderUnitId},
    traits::Scene,
    types::Position3D,
    ui::style::{Align, Justify},
};
use std::{
    marker::PhantomData,
    sync::{Arc, Weak, atomic::AtomicUsize, mpsc},
};

#[derive(Debug)]
pub struct Settings {
    text_handle: Weak<RenderUnitId>,
    init_complete: bool,
}

impl Settings {
    pub fn new() -> Box<dyn Scene> {
        Box::new(Self {
            text_handle: Weak::new(),
            init_complete: false,
        })
    }
}

impl Scene for Settings {
    fn init(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
        signal: Option<Signal>,
        canvas: &Canvas,
        lg: Arc<logging::Logger>,
    ) -> Signal {
        render_tx.send(RenderSignal::Clear);
        self.init_complete = true;
        Signal::None
    }
    fn is_init(&self) -> bool {
        return self.init_complete;
    }
    fn is_paused(&self) -> bool {
        false
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
        let mut signals: Vec<Signal> = vec![];
        for each in event.try_iter() {
            match each {
                Event::Keyboard(k) => match k {
                    KeyEvent::Char('q') => signals.push(Signal::Scenes(SceneSignal::Pop)),
                    _ => {}
                },
                _ => {}
            }
        }
        if signals.len() == 0 {
            return Signal::None;
        } else if signals.len() == 1 {
            return signals.remove(0);
        } else {
            return Signal::Batch(signals);
        }
    }
}
