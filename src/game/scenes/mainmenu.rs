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

use crate::{
    engine::{
        enums::{RenderSignal, SceneSignal, Signal},
        input::{Event, KeyEvent},
        render::{Canvas, Layer, Object, RenderUnitId},
        traits::Scene,
        ui::{
            Border, BorderSprite, Menu, MenuItem, Padding, SelectionDirection, Selector,
            SelectorItem, Textbox,
            style::{self, Align, Justify, Measure, Size, Style},
        },
    },
    game::{LoadGame, Settings},
};
use my_term::color::{Background, Color, Foreground, Iso};
use std::sync::{Arc, Weak, mpsc};

#[allow(unused)]
#[derive(Debug)]
enum Signals {
    None,
    Quit,
    NewScene(Box<dyn Scene>),
}

#[derive(Debug)]
pub struct MainMenu {
    menu: Menu<(), Signals>,
    lg: Option<Arc<logging::Logger>>,
    err_msg_handle: Option<Arc<RenderUnitId>>,
    init_complete: bool,
}

impl MainMenu {
    pub fn new() -> Box<dyn Scene> {
        Box::new(Self {
            menu: Menu::new(
                0,
                0,
                Style::default()
                    .set_border(Border::as_heavy(Padding::square(1)))
                    .set_size(Size::rect(Measure::Percent(50), Measure::Percent(50)))
                    .set_justify(Justify::Center)
                    .set_align(Align::Center)
                    .set_fg(Foreground::green(false))
                    .set_bg(Background::black(false)),
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
            ),
            init_complete: false,
            lg: None,
            err_msg_handle: None,
        })
    }
}

impl Scene for MainMenu {
    fn init(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
        signal: Option<Signal>,
        _canvas: &Canvas,
        lg: Arc<logging::Logger>,
    ) -> Signal {
        self.menu.output(render_tx);
        self.lg = Some(lg);
        self.init_complete = true;
        if let Err(_e) = render_tx.send(RenderSignal::Redraw) {
            // Log that there was a problem
        }
        Signal::None
    }

    fn is_init(&self) -> bool {
        self.init_complete
    }

    fn update(
        &mut self,
        _delta_time: f32,
        event: &mpsc::Receiver<Event>,
        render_tx: &mpsc::Sender<RenderSignal>,
        _canvas: &Canvas,
    ) -> Signal {
        let mut signals: Vec<Signal> = vec![];
        for e in event.try_iter() {
            match e {
                Event::Keyboard(e) => match e {
                    KeyEvent::Up => signals.push(Signal::Render(RenderSignal::ScrollUI(1))),
                    KeyEvent::Down => signals.push(Signal::Render(RenderSignal::ScrollUI(-1))),
                    KeyEvent::Left => signals.push(Signal::Render(RenderSignal::ShiftUI(1))),
                    KeyEvent::Right => signals.push(Signal::Render(RenderSignal::ShiftUI(-1))),
                    KeyEvent::Char('q') => {
                        return Signal::Quit;
                    }
                    KeyEvent::Up | KeyEvent::Char('w') => {
                        if self.menu.cursor_up(1) {
                            self.menu.output(render_tx);
                            let _ = self
                                .lg
                                .as_ref()
                                .unwrap()
                                .write(logging::LogLevel::Info, "MainMenu cursor up 1");
                        }
                    }
                    KeyEvent::Down | KeyEvent::Char('s') => {
                        if self.menu.cursor_down(1) {
                            self.menu.output(render_tx);
                            if let Err(e) = self
                                .lg
                                .as_ref()
                                .unwrap()
                                .write(logging::LogLevel::Info, "MainMenu cursor down 1")
                            {
                                let a: Arc<RenderUnitId> = RenderUnitId::new(Layer::Ui);
                                self.err_msg_handle = Some(a.clone());
                                render_tx.send(RenderSignal::Insert(
                                    a,
                                    Object::static_text(
                                        crate::engine::types::Position3D { x: 2, y: 30, z: 1 },
                                        format!("failed to Log Message from mainmenu: {}", e),
                                        style::Style::default()
                                            .set_border(Border::as_hash(Padding::square(1))),
                                    ),
                                ));
                            }
                        }
                    }
                    KeyEvent::Right | KeyEvent::Char('d') => match self.menu.execute(()) {
                        Signals::Quit => {
                            let _ = self
                                .lg
                                .as_ref()
                                .unwrap()
                                .write(logging::LogLevel::Info, "MainMenu Executing quit action");
                            signals.push(Signal::Quit);
                        }
                        Signals::NewScene(s) => {
                            let _ = self.lg.as_ref().unwrap().write(
                                logging::LogLevel::Info,
                                "MainMenu Executing new scene action",
                            );
                            signals.push(Signal::Scenes(SceneSignal::New {
                                scene: s,
                                signal: None,
                            }));
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

    fn is_paused(&self) -> bool {
        false
    }

    fn reset(&mut self) {}

    fn resume(&mut self, render_tx: &mpsc::Sender<RenderSignal>, _canvas: &Canvas) {
        if let Err(_e) = render_tx.send(RenderSignal::Clear) {
            // Log that there is a problem
        }
        self.menu.output(render_tx);
    }
    #[allow(unused)]
    fn suspend(&mut self, render_tx: &mpsc::Sender<RenderSignal>) {
        if let Err(_e) = render_tx.send(RenderSignal::Clear) {
            // Log that there is a problem
        }
    }
}
