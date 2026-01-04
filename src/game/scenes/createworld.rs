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

use my_term::color::{BLUE, Background, Foreground, GREEN, MAGENTA};

use crate::engine::{
    Instance,
    enums::{RenderSignal, SceneInitSignals, Signal},
    input::{Event, KeyEvent},
    render::Text,
    traits::Scene,
    ui::{
        SelectionDirection, Selector, SelectorItem, TextArea,
        style::{self, MEDIUM_BLOCK},
    },
};
use logging::{ErrorKind as LogErrorKind, LogLevel};

#[derive(Debug)]
pub struct CreateWorld {
    world_name_input: TextArea,
    world_size_input: Selector,
    world_height_delta_input: TextArea,
    world_sea_level_input: TextArea,
    cursor: usize,
    init_complete: bool,
}

impl CreateWorld {
    pub fn new() -> Box<dyn Scene> {
        Box::new(Self {
            cursor: 0,
            world_name_input: TextArea::new(0, 1, MEDIUM_BLOCK, style::Style::default(), None),
            world_size_input: Selector::new(
                0,
                5,
                style::Style::default(),
                style::Coloring {
                    foreground: Foreground::new(GREEN),
                    background: Background::new(my_term::color::RED),
                },
                style::Coloring {
                    foreground: Foreground::new(BLUE),
                    background: Background::new(MAGENTA),
                },
                SelectionDirection::Horizontal,
                vec![
                    SelectorItem::new(
                        Text::from(
                            "Small",
                            style::Style::default().fg(),
                            style::Style::default().bg(),
                        ),
                        0,
                    ),
                    SelectorItem::new(
                        Text::from(
                            "Medium",
                            style::Style::default().fg(),
                            style::Style::default().bg(),
                        ),
                        1,
                    ),
                    SelectorItem::new(
                        Text::from(
                            "Large",
                            style::Style::default().fg(),
                            style::Style::default().bg(),
                        ),
                        2,
                    ),
                ],
            ),
            world_height_delta_input: TextArea::new(
                0,
                9,
                MEDIUM_BLOCK,
                style::Style::default(),
                None,
            ),
            world_sea_level_input: TextArea::new(
                0,
                13,
                MEDIUM_BLOCK,
                style::Style::default(),
                None,
            ),
            init_complete: false,
        })
    }
}

impl Scene for CreateWorld {
    fn init(&mut self, ins: &mut Instance, _sig: SceneInitSignals) -> Signal {
        let lg = ins.logger.clone();
        if let Err(e) = self.world_name_input.output(&ins.render_queue, &ins.canvas) {
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
        if let Err(e) = self.world_size_input.output(&ins.render_queue) {
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
        if let Err(e) = self
            .world_sea_level_input
            .output(&ins.render_queue, &ins.canvas)
        {
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
        if let Err(e) = self
            .world_height_delta_input
            .output(&ins.render_queue, &ins.canvas)
        {
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

    fn reset(&mut self, _ins: &mut Instance) {}

    fn resume(&mut self, ins: &mut Instance) {
        let _ = ins.render_queue.send(RenderSignal::Clear);
        if let Err(_e) = self.world_name_input.output(&ins.render_queue, &ins.canvas) {
            // Log the error
        }
        if let Err(_e) = self.world_size_input.output(&ins.render_queue) {
            // Log the error
        }
        if let Err(_e) = self
            .world_height_delta_input
            .output(&ins.render_queue, &ins.canvas)
        {
            // Log the error
        }
        if let Err(_e) = self
            .world_sea_level_input
            .output(&ins.render_queue, &ins.canvas)
        {
            // Log the error
        }
    }

    fn suspend(&mut self, ins: &mut Instance) {
        let _ = ins.render_queue.send(RenderSignal::Clear);
    }

    fn update(&mut self, inst: &mut Instance, _dt: f32) -> Signal {
        let canvas = &inst.canvas;
        let lg = inst.logger.clone();
        let mut signals: Vec<Signal> = vec![];
        let mut events = vec![];
        for e in inst.event_recvier.try_iter() {
            events.push(e);
        }
        for e in events {
            match e {
                Event::Keyboard(key) => match key {
                    KeyEvent::Char('t') => {
                        let _ = lg.write(LogLevel::Info, "Testing from Create World Scene");
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
                        self.world_name_input
                            .process_key(key, &inst.render_queue, canvas)
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
                    Event::Keyboard(key) => {
                        self.world_height_delta_input
                            .process_key(key, &inst.render_queue, canvas)
                    }
                    _ => {}
                },
                3 => match e {
                    Event::Keyboard(key) => {
                        self.world_sea_level_input
                            .process_key(key, &inst.render_queue, canvas)
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
}
