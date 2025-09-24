use crate::{
    engine::{
        self, input::{Event, KeyEvent}, enums::Signal,
        render::{self, insert_text},
        types::Position,
        ui::{Border, Menu, MenuItem, UIElement},
    },
    game::{Game, LoadGame, Settings},
};

enum Signals {
    Quit,
    NewScene(Game),
}

use std::{io::Write, sync::mpsc};

pub struct MainMenu {
    menu: Menu<MainMenu, Signals>,
    init_complete: bool,
}

impl MainMenu {
    pub fn init(&mut self, render_tx: &mpsc::Sender<render::Msg>) -> Signal<Game> {
        let _ = render_tx.send(render::Msg::InsertText {
            pos: Position::new(self.menu.x(), self.menu.y()),
            text: self.menu.output(),
            prefix: None,
            suffix: None,
        });
        let _ = render_tx.send(render::Msg::Prefix(String::from("\x1b[43m")));
        self.init_complete = true;

        Signal::None
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
                        action: |_g: &MainMenu| -> Option<Signals> {
                            Some(Signals::NewScene(LoadGame::new()))
                        },
                    },
                    MenuItem {
                        label: String::from("Settings"),
                        action: |_g: &MainMenu| -> Option<Signals> {
                            Some(Signals::NewScene(Settings::new()))
                        },
                    },
                    MenuItem {
                        label: String::from("Quit"),
                        action: |_g: &MainMenu| -> Option<Signals> { Some(Signals::Quit) },
                    },
                ],
            ),
            init_complete: false,
        })
    }
    pub fn update(
        &mut self,
        _delta_time: f32,
        event: &mpsc::Receiver<Event>,
        render_tx: &mpsc::Sender<render::Msg>,
    ) -> Signal<Game> {
        let mut signals: Vec<Signal<Game>> = vec![];
        for e in event.try_iter() {
            match e {
                Event::Keyboard(e) => match e {
                    KeyEvent::Char('q') => {
                        return Signal::Quit;
                    }
                    KeyEvent::Char('e') => {}
                    KeyEvent::Char('B') => {
                        let _ = render::insert_text(1, 1, "Term RPG".to_string(), render_tx);
                    }
                    KeyEvent::Up | KeyEvent::Char('w') => {
                        let pre_pos = self.menu.cursor_pos();
                        if self.menu.cursor_up(1) {
                            let _ = render_tx.send(render::Msg::Batch(vec![
                                render::Msg::Remove(pre_pos),
                                render::Msg::Insert(self.menu.cursor_pos(), self.menu.marker_object().unwrap()),
                            ]));
                        }
                    }
                    KeyEvent::Down | KeyEvent::Char('s') => {
                        let pre_pos = self.menu.cursor_pos();
                        if self.menu.cursor_down(1) {
                            let _ = render_tx.send(render::Msg::Batch(vec![
                                render::Msg::Remove(pre_pos),
                                render::Msg::Insert(self.menu.cursor_pos(), self.menu.marker_object().unwrap()),
                            ]));
                        }
                    }
                    KeyEvent::Right | KeyEvent::Char('d') => {
                        if let Some(output) = self.menu.execute(self) {
                            match output {
                                Signals::Quit => {
                                    signals.push(Signal::Quit);
                                }
                                Signals::NewScene(s) => {
                                    signals.push(Signal::NewScene(s));
                                }
                            }
                        }
                    }
                    KeyEvent::Char('c') => {
                        let _ = render_tx.send(render::Msg::InsertText {
                            pos: Position::new(5, 5),
                            text: "This\nIs\nSparta".to_string(),
                            prefix: None,
                            suffix: None,
                        });
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        if signals.len() > 0 {
            return Signal::Batch(signals);
        } else {
            return Signal::None;
        }
    }
    pub fn is_paused(&self) -> bool {
        false
    }
    pub fn reset(&mut self) {}
    pub fn resume(&mut self, render_tx: &mpsc::Sender<render::Msg>) {
        let _ = insert_text(self.menu.x(), self.menu.y(), self.menu.output(), render_tx);
        let _ = render_tx.send(render::Msg::Prefix(String::from("\x1b[43m")));
    }
    pub fn suspend(&mut self, render_tx: &mpsc::Sender<render::Msg>) {
        let _ = render_tx.send(render::Msg::Prefix(String::new()));
    }
}
