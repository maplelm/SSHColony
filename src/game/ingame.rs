use crate::engine::{self, Scene, render::RenderMsg};
use crate::game::Game;
use std::{marker::PhantomData, sync::mpsc};
pub struct InGame {
    init_complete: bool
}

impl InGame {
    pub fn new() -> Game {
        Game::InGame(Self {
            init_complete: false
        })
    }
}

impl InGame {
    pub fn init(&mut self, render_tx: &mpsc::Sender<RenderMsg>) {

        self.init_complete = true;
    }
    pub fn is_init(&self) -> bool {
        self.init_complete
    }
    pub fn is_paused(&self) -> bool {
        false
    }
    pub fn reset(&mut self) {}
    pub fn resume(&mut self) {}
    pub fn suspend(&mut self) {}
    pub fn update(
        &mut self,
        delta_time: f32,
        event: &mpsc::Receiver<engine::Event>,
        render_tx: &std::sync::mpsc::Sender<crate::engine::RenderMsg>,
    ) -> engine::Signal<Game> {
        engine::Signal::None
    }
}

