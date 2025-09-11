#![allow(unused,)]
use crate::engine::{self, Scene};
use super::Game;
use std::{marker::PhantomData, sync::mpsc};

pub struct Settings<_T: Scene<_T>> {
    _marker: PhantomData<_T>
}

impl Settings<Game> {
    pub fn new() -> Game {
        Game::Settings(Self{
            _marker: PhantomData
        })
    }
}

impl<T: Scene<T>> Scene<T> for Settings<T> {
    fn init(&mut self) {
        
    }
    fn is_paused(&self) -> bool {
        false
    }
    fn reset(&mut self) {
        
    }
    fn resume(&mut self) {
        
    }
    fn suspend(&mut self) {
        
    }
    fn update(
            &mut self,
            delta_time: f32,
            event: &mpsc::Receiver<crate::engine::input::Event>,
            render_tx: &std::sync::mpsc::Sender<crate::engine::RenderMsg>,
        ) -> engine::Signal<T> {
            engine::Signal::None
    }
}