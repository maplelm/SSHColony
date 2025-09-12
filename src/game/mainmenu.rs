use super::Game;
use crate::engine::{
    self, render::{insert_plain_text, ObjectPos, RenderMsg}, ui::{Border, Menu, MenuItem, UIElement}, Event, KeyEvent, Scene, Signal
};

enum Signals {
    Quit
}

use std::{io::Write, marker::PhantomData, sync::mpsc};

pub struct MainMenu {
    //menu: ui::Menu<u32>,
    menu: Menu<MainMenu, Signals>,
    init_complete: bool,
    //_marker: PhantomData<T>,
}

impl MainMenu {

    pub fn init(&mut self, render_tx: &mpsc::Sender<RenderMsg>) {
        let _ = render_tx.send(RenderMsg::InsertText(ObjectPos { x: self.menu.x(), y: self.menu.y() }, self.menu.output(), None, None)); 
        self.init_complete = true;
    }

    pub fn is_init(&self) -> bool {
        self.init_complete
    }

    pub fn new() -> Game {
        Game::MainMenu(Self {
            menu: Menu::<MainMenu, Signals>::new(
                1,
                1,
                Some(Border::default()),
                vec![
                    MenuItem {
                        label: String::from("Play"),
                        action: |g: &MainMenu| -> Option<Signals> { None },
                    },
                    MenuItem {
                        label: String::from("Settings"),
                        action: |g: &MainMenu| -> Option<Signals> { None },
                    },
                    MenuItem {
                        label: String::from("Quit"),
                        action: |g: &MainMenu| -> Option<Signals> { Some(Signals::Quit) },
                    },
                ],
            ),
            init_complete: false,
        })
    }
    pub fn update(
        &mut self,
        delta_time: f32,
        event: &mpsc::Receiver<Event>,
        render_tx: &mpsc::Sender<engine::RenderMsg>,
    ) -> engine::Signal<Game> {
        let mut signals: Vec<Signal<Game>> = vec![];
        for e in event.try_iter() {
            match e {
                Event::Keyboard(e) => match e {
                    KeyEvent::Char('q') => {
                        return engine::Signal::Quit;
                    }
                    KeyEvent::Char('e') => {
                    }
                    KeyEvent::Char('B') => {
                        let _ = insert_plain_text(1, 1, "Term RPG".to_string(), render_tx);
                    }
                    KeyEvent::Up | KeyEvent::Char('w') => {
                        let pre_pos = self.menu.cursor_pos();
                        self.menu.cursor_up(1);
                        let post_pos = self.menu.cursor_pos();
                        #[cfg(debug_assertions)]
                        {
                            let mut file = std::fs::OpenOptions::new()
                                .create(true)   // create if it doesn’t exist
                                .append(true)   // always write at end
                                .open("output.log").unwrap();
                            let data = format!("Pre: {:?}  Post: {:?}\n\r", pre_pos, post_pos);
                            file.write_all(data.as_bytes());
                        }
                        if pre_pos.x != post_pos.x || pre_pos.y != post_pos.y {
                            let _ = render_tx.send(RenderMsg::Batch(vec![RenderMsg::Remove(pre_pos), RenderMsg::Insert(post_pos, self.menu.marker_object())]));
                        }
                    }
                    KeyEvent::Down | KeyEvent::Char('s') => {
                        let pre_pos = self.menu.cursor_pos();
                        self.menu.cursor_down(1);
                        let post_pos = self.menu.cursor_pos();
                        #[cfg(debug_assertions)]
                        {
                            let mut file = std::fs::OpenOptions::new()
                                .create(true)   // create if it doesn’t exist
                                .append(true)   // always write at end
                                .open("output.log").unwrap();
                            let data = format!("Pre: {:?}  Post: {:?}\n\r", pre_pos, post_pos);
                            file.write_all(data.as_bytes());
                        }
                        if pre_pos != post_pos {
                            let _ = render_tx.send(RenderMsg::Batch(vec![RenderMsg::Remove(pre_pos), RenderMsg::Insert(post_pos, self.menu.marker_object())]));
                            //let _ = render_tx.send(RenderMsg::Swap(pre_pos, post_pos));
                        }
                    }
                    KeyEvent::Right | KeyEvent::Char('d')=> {
                        if let Some(output) = self.menu.execute(self) {
                            match output {
                                Signals::Quit => {
                                    signals.push(engine::Signal::Quit);
                                }
                            }
                        }
                    }
                    KeyEvent::Char('c') => {
                        let _ = render_tx.send(RenderMsg::InsertText(
                            ObjectPos { x: 5, y: 5 },
                            "This\nIs\nSparta".to_string(),
                            None,
                            None,
                        ));
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        if signals.len() > 0 {
            return engine::Signal::Batch(signals);
        } else {
            return engine::Signal::None;
        }
    }
    pub fn is_paused(&self) -> bool {
        false
    }
    pub fn reset(&mut self) {}
    pub fn resume(&mut self) {}
    pub fn suspend(&mut self) {}
}
