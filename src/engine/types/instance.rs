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

use super::super::core::traits::Scene;
use super::super::{Context, consts::DEFAULT_CANVAS, render::Canvas};
use logging::Logger;
use logging::Options as Opts;
use my_term::{Terminal, term_size};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct Instance {
    pub ctx: Context,
    pub term_orig: Terminal,
    pub game_state: Vec<Box<dyn Scene>>,
    pub canvas: Canvas,
    pub logger: Arc<Logger>,
}

impl Default for Instance {
    fn default() -> Self {
        let mut canvas = DEFAULT_CANVAS;
        if let Some(size) = term_size() {
            canvas.width = size.0 as usize;
            canvas.height = size.1 as usize;
        }
        let mut logpath = PathBuf::new();
        logpath.set_file_name("./logs/");

        Self {
            ctx: Context::new(),
            term_orig: Terminal::default(),
            game_state: vec![],
            canvas: canvas,
            logger: Arc::new(Logger::new(Opts::default()).unwrap()),
        }
    }
}

impl Instance {
    pub fn new(init_scene: Box<dyn Scene>, log_path: &str, log_level: logging::LogLevel) -> Self {
        let mut canvas = DEFAULT_CANVAS;
        if let Some(size) = term_size() {
            canvas.width = size.0 as usize;
            canvas.height = size.1 as usize;
        }
        Self {
            ctx: Context::new(),
            term_orig: Terminal::default(),
            game_state: vec![init_scene],
            canvas: canvas,
            logger: Arc::new(Logger::new(Opts::default().set_lvl(log_level)).unwrap()),
        }
    }

    pub fn add_scene(&mut self, s: Box<dyn Scene>) {
        self.game_state.push(s);
    }
}
