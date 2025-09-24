#![allow(unused)]
use crate::{
    engine::{self, render, input::{Event, KeyEvent}, enums::Signal, traits::Scene},
    game::Game
};
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
    pub fn init(&mut self, render_tx: &mpsc::Sender<render::Msg>) -> Signal<Game> {
       render_tx.send(render::Msg::Clear);
       render::insert_text(1, 1, "Settings!".to_string(), render_tx); 
       self.init_complete = true;

       Signal::None
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
        ) -> Signal<Game> {
            let mut signals: Vec<Signal<Game>> = vec![]; 
            for each in event.try_iter() {
                match each {
                    Event::Keyboard(k) => {
                        match k {
                            KeyEvent::Char('q') => signals.push(Signal::PopScene),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if signals.len() == 0 {
                return Signal::None;
            } else if signals.len() == 1  {
                return signals.remove(0);
            } else {
                return Signal::Batch(signals);
            }

    }
}