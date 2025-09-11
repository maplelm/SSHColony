use crate::engine::{self, input::{Event, KeyEvent, MouseEvent, OtherEvent}, render::{insert_plain_text, ObjectPos}, Scene};
use crate::scenes::Game;

use std::{marker::PhantomData, sync::mpsc};


pub struct MainMenu<T: Scene<T>> {
    _marker: PhantomData<T>
}

impl MainMenu<Game> {
    pub fn new() -> Game {
        Game::MainMenu(Self{
            _marker: PhantomData
        })
    }
}

impl<T: Scene<T>> Scene<T> for MainMenu<T> {
    fn init(&mut self) {
        
    }
    fn update (&mut self, delta_time: f32, 
        event: &mpsc::Receiver<crate::engine::input::Event>,
        render_tx: &std::sync::mpsc::Sender<crate::engine::RenderMsg>) -> engine::Signal<T>{
            for e in event.try_iter() {
                match e {
                    Event::Keyboard(e) => {
                        match e {
                            KeyEvent::Char('q') => {
                                return engine::Signal::Quit;
                            }
                            KeyEvent::Char('B') => {
                                let _ = insert_plain_text(1, 1, "Term RPG".to_string(), render_tx);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
       engine::Signal::None 
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
}