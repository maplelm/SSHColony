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
        Instance,
        enums::{RenderSignal, SceneInitSignals, SceneSignal, Signal as EngineSignal},
        input::{Event, KeyEvent},
        render::{Canvas, Layer, Object, ObjectData, RenderQueue, RenderUnitId, Text, TextType},
        traits::Scene,
        types::Position3D,
        ui::{
            Border, BorderSprite, Menu, MenuItem, Padding, SelectionDirection, Selector,
            SelectorItem, TextArea,
            style::{self, Align, Justify, Measure, Size, Style},
        },
    },
    game::{LoadGame, Settings},
};
use my_term::color::{BLACK, Background, Foreground, GREEN};
use std::io::Read;
use std::io::Write;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::TcpStream;
use std::sync::{Arc, Weak, mpsc};

/// Internal Signals for the MainMenu
enum _Sig {
    None,
    Quit,
    Connect,
    MenuCursorDown(usize),
    MenuCursorUp(usize),
    MenuUp(usize),
    MenuDown(usize),
    MenuRight(usize),
    MenuLeft(usize),
    MenuExe,
    Render(RenderSignal),
    Batch(Vec<_Sig>),
    LoadGameScene,
    SettingsScene,
}

pub struct MainMenu {
    menu: Menu<_Sig>,
    err_msg_handle: Option<Arc<RenderUnitId>>,
    init_complete: bool,
}

impl MainMenu {
    pub fn new(render_queue: RenderQueue) -> Box<dyn Scene> {
        Box::new(Self {
            menu: Menu::new(
                0,
                0,
                render_queue,
                Style::default()
                    .set_border(Border::as_heavy(Padding::square(1)))
                    .set_size(Size::rect(Measure::Percent(50), Measure::Percent(50)))
                    .set_justify(Justify::Center)
                    .set_align(Align::Center)
                    .set_fg(Foreground::new(GREEN))
                    .set_bg(Background::new(BLACK)),
                vec![
                    MenuItem {
                        label: Text::from(
                            "Connect",
                            Foreground::new(GREEN),
                            Background::new(BLACK),
                        ),
                        action: action_connect,
                    },
                    MenuItem {
                        label: Text::from(
                            "Settings",
                            Foreground::new(GREEN),
                            Background::new(BLACK),
                        ),
                        action: action_goto_settings,
                    },
                    MenuItem {
                        label: Text::from("Quit", Foreground::new(GREEN), Background::new(BLACK)),
                        action: action_quit,
                    },
                ],
            ),
            init_complete: false,
            err_msg_handle: None,
        })
    }

    fn process_event(&mut self, e: Event) -> _Sig {
        match e {
            Event::Keyboard(e) => match e {
                KeyEvent::Up => _Sig::Render(RenderSignal::ScrollUI(-1)),
                KeyEvent::Down => _Sig::Render(RenderSignal::ScrollUI(1)),
                KeyEvent::Left => _Sig::Render(RenderSignal::ShiftUI(-1)),
                KeyEvent::Right => _Sig::Render(RenderSignal::ShiftUI(1)),
                KeyEvent::Char('W') => _Sig::MenuUp(1),
                KeyEvent::Char('S') => _Sig::MenuDown(1),
                KeyEvent::Char('A') => _Sig::MenuLeft(1),
                KeyEvent::Char('D') => _Sig::MenuRight(1),
                KeyEvent::Char('q') => _Sig::Quit,
                KeyEvent::Char('w') => _Sig::MenuCursorUp(1),
                KeyEvent::Char('s') => _Sig::MenuCursorDown(1),
                KeyEvent::Char('d') => _Sig::MenuExe,
                _ => _Sig::None,
            },
            _ => _Sig::None,
        }
    }

    fn process_signal(&mut self, inst: &mut Instance, s: _Sig) -> EngineSignal {
        match s {
            _Sig::Batch(batch) => {
                let mut output = vec![];
                for each in batch {
                    match self.process_signal(inst, each) {
                        EngineSignal::None => {}
                        other => output.push(other),
                    };
                }
                if output.len() == 0 {
                    EngineSignal::None
                } else if output.len() == 1 {
                    output.pop().unwrap()
                } else {
                    EngineSignal::Batch(output)
                }
            }
            _Sig::Quit => EngineSignal::Quit,
            _Sig::LoadGameScene => EngineSignal::Scenes(SceneSignal::New {
                scene: LoadGame::new(inst.render_queue.clone()),
                signal: SceneInitSignals::None,
            }),
            _Sig::SettingsScene => EngineSignal::Scenes(SceneSignal::New {
                scene: Settings::new(),
                signal: SceneInitSignals::None,
            }),
            _Sig::Connect => {
                let (serv_ver, tick_rate) = match inst.net.send_hel() {
                    Ok(res) => res,
                    Err(e) => {

                        let _ = inst.logger.write(logging::LogLevel::Error, "Failed to connect to game server");
                        return EngineSignal::None;
                    }
                };
                
                inst.tick_rate = tick_rate;
                EngineSignal::None
            }
            _Sig::Render(r) => EngineSignal::Render(r),
            _Sig::MenuCursorUp(d) => {
                self.menu.cursor_up(d);
                EngineSignal::None
            }
            _Sig::MenuCursorDown(d) => {
                self.menu.cursor_down(d);
                EngineSignal::None
            }
            _Sig::MenuRight(d) => {
                self.menu.shift(d as i32, 0);
                EngineSignal::None
            }
            _Sig::MenuLeft(d) => {
                self.menu.shift(-(d as i32), 0);
                EngineSignal::None
            }
            _Sig::MenuUp(d) => {
                self.menu.shift(0, -(d as i32));
                EngineSignal::None
            }
            _Sig::MenuDown(d) => {
                self.menu.shift(0, d as i32);
                EngineSignal::None
            }
            _Sig::MenuExe => {
                let sig = self.menu.execute();
                self.process_signal(inst, sig)
            }
            _Sig::None => EngineSignal::None,
        }
    }
}

impl Scene for MainMenu {
    fn init(&mut self, inst: &mut Instance, signal: SceneInitSignals) -> EngineSignal {
        self.menu.output();
        self.init_complete = true;
        EngineSignal::Render(RenderSignal::Redraw)
    }
    fn is_init(&self) -> bool {
        self.init_complete
    }

    fn update(&mut self, inst: &mut Instance, _delta_time: f32) -> EngineSignal {
        let mut output = vec![];
        let mut events = vec![];
        for e in inst.event_recvier.try_iter() {
            events.push(e);
        }
        for e in events {
            let sig = self.process_event(e);
            match self.process_signal(inst, sig) {
                EngineSignal::None => {}
                other => output.push(other),
            };
        }
        if output.len() == 0 {
            EngineSignal::None
        } else if output.len() == 1 {
            output.pop().unwrap()
        } else {
            EngineSignal::Batch(output)
        }
    }

    fn is_paused(&self) -> bool {
        false
    }

    fn reset(&mut self, ins: &mut Instance) {}

    fn resume(&mut self, ins: &mut Instance) {
        if let Err(_e) = ins.render_queue.send(RenderSignal::Clear) {
            // Log that there is a problem
        }
        self.menu.output();
    }
    fn suspend(&mut self, ins: &mut Instance) {
        if let Err(_e) = ins.render_queue.send(RenderSignal::Clear) {
            // Log that there is a problem
        }
    }
}

////////////////////
//  Menu Actions  //
////////////////////

fn action_connect() -> _Sig {
    _Sig::Batch(vec![_Sig::Connect, _Sig::LoadGameScene])
}

fn action_goto_settings() -> _Sig {
    _Sig::SettingsScene
}

fn action_quit() -> _Sig {
    _Sig::Quit
}