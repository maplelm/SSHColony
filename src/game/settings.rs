#![allow(unused,)]
use crate::engine::{self, Scene, Event, render::RenderMsg};
use super::Game;
use std::{marker::PhantomData, sync::mpsc};

pub struct Settings {
    init_complete: bool
}

impl Settings {
    pub fn new() -> Game {
        Game::Settings(Self{
            init_complete: false
        })
    }
}

impl Settings {
    pub fn init(&mut self, render_tx: &mpsc::Sender<RenderMsg>) {
        
    }
    pub fn is_init(&self) -> bool {
        return self.init_complete;
    }
    pub fn is_paused(&self) -> bool {
        false
    }
    pub fn reset(&mut self) {
        
    }
    pub fn resume(&mut self) {
        
    }
    pub fn suspend(&mut self) {
        
    }
    pub fn update(
            &mut self,
            delta_time: f32,
            event: &mpsc::Receiver<Event>,
            render_tx: &std::sync::mpsc::Sender<crate::engine::RenderMsg>,
        ) -> engine::Signal<Game> {
            engine::Signal::None
    }
}