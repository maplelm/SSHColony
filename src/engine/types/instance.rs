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

use crate::engine::input::Event;
use crate::engine::types::Network;

use super::super::core::traits::Scene;
use super::super::{
    Context,
    consts::DEFAULT_CANVAS,
    render::{Canvas, RenderQueue},
};
use logging::Logger;
use logging::Options as Opts;
use my_term::{Terminal, term_size};
use rand_core::OsRng;
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

pub struct InstanceConfig {
    log_path: String,
    log_level: logging::LogLevel,
}

impl InstanceConfig {
    pub fn new(path: impl Into<String>, level: logging::LogLevel) -> Self {
        Self {
            log_path: path.into(),
            log_level: level,
        }
    }
}

pub struct Instance {
    pub ctx: Context,
    pub term_orig: Terminal,
    pub canvas: Canvas,
    pub logger: Arc<Logger>,
    pub net: Network,
    pub render_queue: RenderQueue,
    pub event_recvier: mpsc::Receiver<Event>,
    pub tick_rate: u16,
}

impl Instance {
    pub fn new(
        config: InstanceConfig,
        queue: RenderQueue,
        event_rx: mpsc::Receiver<Event>,
    ) -> Self {
        let mut canvas = DEFAULT_CANVAS;
        if let Some(size) = term_size() {
            canvas.width = size.0 as usize;
            canvas.height = size.1 as usize;
        }
        Self {
            ctx: Context::new(),
            term_orig: Terminal::default(),
            canvas: canvas,
            logger: Arc::new(
                Logger::new(
                    Opts::default()
                        .set_lvl(config.log_level)
                        .set_path(config.log_path.as_str()),
                )
                .unwrap(),
            ),
            net: Network::default(),
            render_queue: queue,
            event_recvier: event_rx,
            tick_rate: 0,
        }
    }
}
