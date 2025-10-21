/*
Copyright 2025 Luke Maple

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
you may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use term::color::{Background, Color, Foreground, Iso};
use std::sync::mpsc;
use crate::{
    engine::{
        enums::{RenderSignal, SceneSignal, Signal},
        input::{Event, KeyEvent},
        render::Canvas,
        ui::{
            Border, BorderSprite, Menu, MenuItem, Padding, SelectionDirection, Selector,
            SelectorItem, Textbox,
            style::{Align, Justify, Measure, Origin, Style},
        },
    },
    game::{Game, LoadGame, Settings},
};

#[allow(unused)]
enum Signals {
    None,
    Quit,
    NewScene(Game),
}


pub struct MainMenu {
    menu: Menu<(), Signals>,
    init_complete: bool,
}

impl MainMenu {
    pub fn new() -> Game {
        Game::MainMenu(Self {
            menu: Menu::new(
                0,
                0,
                Some(Measure::Percent(100)),
                Some(Measure::Percent(100)),
                Origin::TopLeft,
                Justify::Center,
                Align::Center,
                Some(
                    Border::from(BorderSprite::String("|#".to_string()), Padding::square(2))
                        .top(BorderSprite::String("-#".to_string()))
                        .bottom(BorderSprite::String("-#".to_string())),
                ),
                vec![
                    MenuItem {
                        label: String::from("Play"),
                        action: |_| -> Signals { Signals::NewScene(LoadGame::new()) },
                    },
                    MenuItem {
                        label: String::from("Settings"),
                        action: |_| -> Signals { Signals::NewScene(Settings::new()) },
                    },
                    MenuItem {
                        label: String::from("Quit"),
                        action: |_| -> Signals { Signals::Quit },
                    },
                ],
                Some(Foreground::new(Color::Iso {
                    color: Iso::Green,
                    bright: false,
                })),
                Some(Background::new(Color::Iso {
                    color: Iso::Black,
                    bright: false,
                })),
            ),
            init_complete: false,
        })
    }

    pub fn init(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
        _canvas: &Canvas,
    ) -> Signal<Game> {
        self.menu.output(render_tx);
        self.init_complete = true;
        if let Err(_e) = render_tx.send(RenderSignal::Redraw) {
            // Log that there was a problem
        }
        Signal::None
    }

    pub fn is_init(&self) -> bool {
        self.init_complete
    }

    pub fn update(
        &mut self,
        _delta_time: f32,
        event: &mpsc::Receiver<Event>,
        render_tx: &mpsc::Sender<RenderSignal>,
        _canvas: &Canvas,
    ) -> Signal<Game> {
        let mut signals: Vec<Signal<Game>> = vec![];
        for e in event.try_iter() {
            match e {
                Event::Keyboard(e) => match e {
                    KeyEvent::Char('q') => {
                        return Signal::Quit;
                    }
                    KeyEvent::Up | KeyEvent::Char('w') => {
                        if self.menu.cursor_up(1) {
                            self.menu.output(render_tx);
                        }
                    }
                    KeyEvent::Down | KeyEvent::Char('s') => {
                        if self.menu.cursor_down(1) {
                            self.menu.output(render_tx);
                        }
                    }
                    KeyEvent::Right | KeyEvent::Char('d') => match self.menu.execute(()) {
                        Signals::Quit => {
                            signals.push(Signal::Quit);
                        }
                        Signals::NewScene(s) => {
                            signals.push(Signal::Scenes(SceneSignal::New(s)));
                        }
                        Signals::None => (),
                    },
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

    pub fn resume(&mut self, render_tx: &mpsc::Sender<RenderSignal>, _canvas: &Canvas) {
        if let Err(_e) = render_tx.send(RenderSignal::Clear) {
            // Log that there is a problem
        }
        self.menu.output(render_tx);
    }
    #[allow(unused)]
    pub fn suspend(&mut self, render_tx: &mpsc::Sender<RenderSignal>) {
        if let Err(_e) = render_tx.send(RenderSignal::Clear) {
            // Log that there is a problem
        }
    }
}
