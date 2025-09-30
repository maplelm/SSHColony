#![deny(unused)]


use crate::{
    engine::{
        enums::Signal,
        input::{Event, KeyEvent},
        render::{self, insert_text, Canvas},
        types::Position,
        ui::{
            style::{Justify, Origin}, Border, BorderSprite, Menu, MenuItem, Output, Padding
        },
    },
    game::{Game, LoadGame, Settings},
};
use std::sync::mpsc::Sender;

enum Signals {
    Quit,
    NewScene(Game),
}

use std::sync::mpsc;

pub struct MainMenu {
    menu: Menu,
    init_complete: bool,
}

impl MainMenu {
    pub fn new() -> Game {
        Game::MainMenu(Self {
            menu: Menu::new(
                0,
                1,
                None, //Some(Measure::Cell(10)),
                None, //Some(Measure::Cell(15)),
                Origin::TopLeft,
                Justify::Left,
                Some(Border::from(
                    BorderSprite::String("|#".to_string()),
                    Padding::square(2),
                )),
                vec![
                    LoadGameItem::new("Play"),
                    MenuItem {
                        label: String::from("Settings"),
                        action: |_| -> Option<Signals> { Some(Signals::NewScene(Settings::new())) },
                    },
                    MenuItem {
                        label: String::from("Quit"),
                        action: |_| -> Option<Signals> { Some(Signals::Quit) },
                    },
                ],
            ),
            init_complete: false,
        })
    }

    pub fn init(&mut self, render_tx: &mpsc::Sender<render::Msg>, canvas: &Canvas) -> Signal<Game> {
        if let Some(out) = self.menu.output(canvas) {
            let _ = render_tx.send(render::Msg::InsertText {
                pos: Position::new(self.menu.x(), self.menu.y()),
                text: out,
                prefix: None,
                suffix: None,
            });
        }
        self.init_complete = true;

        Signal::None
    }

    pub fn is_init(&self) -> bool {
        self.init_complete
    }

    pub fn update(
        &mut self,
        _delta_time: f32,
        event: &mpsc::Receiver<Event>,
        render_tx: &mpsc::Sender<render::Msg>,
        _canvas: &Canvas,
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
                        let pre_pos = { self.menu.cursor_pos() };
                        if self.menu.cursor_up(1) {
                            let _ = render_tx.send(render::Msg::Batch(vec![
                                render::Msg::Remove(pre_pos),
                                render::Msg::Insert(
                                    self.menu.cursor_pos(),
                                    self.menu.marker_object(),
                                ),
                            ]));
                        }
                    }
                    KeyEvent::Down | KeyEvent::Char('s') => {
                        let pre_pos = self.menu.cursor_pos();
                        if self.menu.cursor_down(1) {
                            let _ = render_tx.send(render::Msg::Batch(vec![
                                render::Msg::Remove(pre_pos),
                                render::Msg::Insert(
                                    self.menu.cursor_pos(),
                                    self.menu.marker_object(),
                                ),
                            ]));
                        }
                    }
                    KeyEvent::Right | KeyEvent::Char('d') => {
                        if let Some(output) = self.menu.execute(()) {
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
    pub fn resume(&mut self, render_tx: &mpsc::Sender<render::Msg>, canvas: &Canvas) {
        if let Some(out) = self.menu.output(canvas) {
            let _ = insert_text(self.menu.x(), self.menu.y(), out, render_tx);
        }
    }
    #[allow(unused)]
    pub fn suspend(&mut self, render_tx: &mpsc::Sender<render::Msg>) {}
}

//crate::menu_item_scene_push!{Game, LoadGame::new(), LoadGameItem}


//crate::menu_item_scene_push!{Game, Settings::new(), SettingsItem}

//crate::new_scene!{Game, TestingThisMacro, {Signal::None}, true, false, {Signal::None}, {}, {}, {}, testing{name: String, test: bool, x: u32, y: u32}}