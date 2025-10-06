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
