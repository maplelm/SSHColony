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
#![deny(unused)]

use my_term::color::{Background, Foreground};

use crate::engine::{
    enums::{RenderSignal, Signal},
    input::{Event, KeyEvent},
    render::Canvas,
    traits::Scene,
    ui::{
        SelectionDirection, Selector, SelectorItem, Textbox,
        style::{self, MEDIUM_BLOCK},
    },
};
use logging::{ErrorKind as LogErrorKind, LogLevel};
use std::sync::{
    Arc,
    mpsc::{Receiver, Sender},
};

#[derive(Debug)]
pub struct CreateWorld {
    world_name_input: Textbox,
    world_size_input: Selector,
    world_height_delta_input: Textbox,
    world_sea_level_input: Textbox,
    cursor: usize,
    init_complete: bool,
    lg: Option<Arc<logging::Logger>>,
}

impl CreateWorld {
    pub fn new() -> Box<dyn Scene> {
        Box::new(Self {
            cursor: 0,
            world_name_input: Textbox::new(0, 1, MEDIUM_BLOCK, Some(style::Style::default()), None),
            world_size_input: Selector::new(
                0,
                5,
                style::Style::default(),
                style::Coloring {
                    foreground: Some(Foreground::green(true)),
                    background: Some(Background::red(false)),
                },
                style::Coloring {
                    foreground: Some(Foreground::blue(false)),
                    background: Some(Background::magenta(true)),
                },
                SelectionDirection::Horizontal,
                vec![
                    SelectorItem::new("Small".to_string(), 0),
                    SelectorItem::new("Medium".to_string(), 1),
                    SelectorItem::new("Large".to_string(), 2),
                ],
            ),
            world_height_delta_input: Textbox::new(
                0,
                9,
                MEDIUM_BLOCK,
                Some(style::Style::default()),
                None,
            ),
            world_sea_level_input: Textbox::new(
                0,
                13,
                MEDIUM_BLOCK,
                Some(style::Style::default()),
                None,
            ),
            init_complete: false,
            lg: None,
        })
    }
}

impl Scene for CreateWorld {
    fn init(
        &mut self,
        render_tx: &Sender<RenderSignal>,
        _signal: Option<Signal>,
        _canvas: &Canvas,
        lg: Arc<logging::Logger>,
    ) -> Signal {
        self.lg = Some(lg);
        let lg = self.lg.as_ref().unwrap();
        if let Err(e) = self.world_name_input.output(render_tx) {
            match lg.write(
                LogLevel::Error,
                format!("Error outputting World Name Input: {}", e),
            ) {
                Err(e) => match e.kind {
                    LogErrorKind::Level => {}
                    _ => panic!("Failed to log: {}", e.msg),
                },
                Ok(_) => {}
            };
        }
        if let Err(e) = self.world_size_input.output(render_tx) {
            match lg.write(
                LogLevel::Error,
                format!("Error outputting World Size Input: {}", e),
            ) {
                Ok(_) => {}
                Err(e) => match e.kind {
                    LogErrorKind::Level => {}
                    _ => panic!("Failed to Log: {}", e.msg),
                },
            };
        }
        if let Err(e) = self.world_sea_level_input.output(render_tx) {
            match lg.write(
                LogLevel::Error,
                format!("Error outputting World Sea Level Input: {}", e),
            ) {
                Ok(_) => {}
                Err(e) => match e.kind {
                    LogErrorKind::Level => {}
                    _ => panic!("Failed to log: {}", e.msg),
                },
            };
        }
        if let Err(e) = self.world_height_delta_input.output(render_tx) {
            match lg.write(
                LogLevel::Error,
                format!("Error outputting World delta Input: {}", e),
            ) {
                Ok(_) => {}
                Err(e) => match e.kind {
                    LogErrorKind::Level => {}
                    _ => panic!("Failed to Log: {}", e.msg),
                },
            };
        }
        self.init_complete = true;
        Signal::None
    }

    fn is_init(&self) -> bool {
        self.init_complete
    }

    fn is_paused(&self) -> bool {
        false
    }

    fn reset(&mut self) {}

    fn resume(&mut self, render_tx: &Sender<RenderSignal>, _canvas: &Canvas) {
        let _ = render_tx.send(RenderSignal::Clear);
        if let Err(_e) = self.world_name_input.output(render_tx) {
            // Log the error
        }
        if let Err(_e) = self.world_size_input.output(render_tx) {
            // Log the error
        }
        if let Err(_e) = self.world_height_delta_input.output(render_tx) {
            // Log the error
        }
        if let Err(_e) = self.world_sea_level_input.output(render_tx) {
            // Log the error
        }
    }

    fn suspend(&mut self, render_tx: &Sender<RenderSignal>) {
        let _ = render_tx.send(RenderSignal::Clear);
    }

    fn update(
        &mut self,
        _dt: f32,
        event: &Receiver<Event>,
        render_tx: &Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> Signal {
        let mut signals: Vec<Signal> = vec![];
        for e in event.try_iter() {
            match e {
                Event::Keyboard(key) => match key {
                    KeyEvent::Char('t') => {
                        let _ = self
                            .lg
                            .as_ref()
                            .unwrap()
                            .write(LogLevel::Info, "Testing from Create World Scene");
                    }
                    KeyEvent::Char('q') => {
                        signals.push(Signal::Scenes(crate::engine::enums::SceneSignal::Pop))
                    }
                    KeyEvent::Tab => {
                        self.cursor = (self.cursor + 1) % 4;
                    }
                    _other => {}
                },
                _ => {}
            };
            match self.cursor {
                0 => match e {
                    Event::Keyboard(key) => {
                        self.world_name_input.process_key(key, render_tx, canvas)
                    }
                    _ => {}
                },
                1 => match e {
                    Event::Keyboard(key) => match key {
                        KeyEvent::Char('a') => self.world_size_input.prev(),
                        KeyEvent::Char('d') => self.world_size_input.next(),
                        KeyEvent::Enter => self.world_size_input.toggle_select(),
                        _ => {}
                    },
                    _ => {}
                },
                2 => match e {
                    Event::Keyboard(key) => self
                        .world_height_delta_input
                        .process_key(key, render_tx, canvas),
                    _ => {}
                },
                3 => match e {
                    Event::Keyboard(key) => self
                        .world_sea_level_input
                        .process_key(key, render_tx, canvas),
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
}
