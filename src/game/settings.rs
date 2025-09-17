#![allow(unused,)]
use crate::engine::{self, render, Event, Scene};
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
    pub fn init(&mut self, render_tx: &mpsc::Sender<render::Msg>) {
       render_tx.send(render::Msg::Clear);
       render::insert_text(1, 1, "Settings!".to_string(), render_tx); 
       self.init_complete = true;
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
            render_tx: &std::sync::mpsc::Sender<crate::engine::render::Msg>,
        ) -> engine::Signal<Game> {
            let mut signals: Vec<engine::Signal<Game>> = vec![]; 
            for each in event.try_iter() {
                match each {
                    Event::Keyboard(k) => {
                        match k {
                            engine::KeyEvent::Char('q') => signals.push(engine::Signal::PopScene),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if signals.len() == 0 {
                return engine::Signal::None;
            } else if signals.len() == 1  {
                return signals.remove(0);
            } else {
                return engine::Signal::Batch(signals);
            }

    }
}