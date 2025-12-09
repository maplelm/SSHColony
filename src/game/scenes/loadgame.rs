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

use crate::engine::enums::RenderSignal;
use crate::engine::enums::Signal as EngineSignal;
use crate::engine::render::clear as render_clear;
use crate::engine::traits::Scene;
use crate::engine::ui::BorderSprite as Bsprite;
use crate::engine::ui::MenuItem;
use crate::engine::ui::style::Size;
use crate::engine::ui::style::Style;
use crate::game::scenes::CreateWorld;
use crate::game::scenes::PlayGame;

use super::super::types::World;
use crate::engine::input::{Event, KeyEvent};
use crate::engine::{
    render::Canvas,
    ui::{
        Border, Menu, Padding,
        style::{Align, Justify, Measure},
    },
};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::sync::{
    Arc,
    mpsc::{Receiver, Sender},
};

#[derive(Debug)]
enum Signal {
    None,
    NewWorld,
    LoadWorld(String),
    WorldData(World),
    Back,
}

enum Intensity {
    Low,
    Medium,
    High,
    Critical,
}

struct NewWorldForm {
    world_name: String,
    seed: u64,
    avg_temp: Intensity,
    avg_height: Intensity,
    height_delta: Intensity,
    hard_core: bool,
}

#[derive(Debug)]
pub struct LoadGame {
    menu: Menu<(), Signal>,
    is_init: bool,
}
impl LoadGame {
    pub fn new() -> Box<dyn Scene> {
        Box::new(Self {
            menu: Menu::<(), Signal>::new(
                0,
                0,
                Style::default()
                    .set_size(Size::rect(Measure::Percent(100), Measure::Percent(100)))
                    .set_justify(Justify::Center)
                    .set_border(Border::as_block(Padding::square(1))),
                vec![
                    MenuItem::new("Play Now".to_string(), |z| Signal::NewWorld),
                    MenuItem::new("Back".to_string(), |z| Signal::Back),
                ],
            ),
            is_init: false,
        })
    }
}

impl Scene for LoadGame {
    fn init(
        &mut self,
        render_tx: &Sender<RenderSignal>,
        signal: Option<EngineSignal>,
        canvas: &Canvas,
        lg: Arc<logging::Logger>,
    ) -> EngineSignal {
        if let Err(_e) = render_clear(render_tx) {
            // log that there was a problem clearing the screen
        }
        self.menu.output(render_tx);
        EngineSignal::None
    }

    fn is_init(&self) -> bool {
        true
    }

    fn is_paused(&self) -> bool {
        false
    }

    fn reset(&mut self) {}

    fn resume(&mut self, render_tx: &Sender<RenderSignal>, canvas: &Canvas) {
        if let Err(_e) = render_tx.send(RenderSignal::Clear) {
            // Log that there was an error
        }
        self.menu.output(render_tx);
    }

    fn suspend(&mut self, render_tx: &Sender<RenderSignal>) {
        if let Err(_e) = render_tx.send(RenderSignal::Clear) {
            // Log that there was an error
        }
    }

    fn update(
        &mut self,
        delta_time: f32,
        event: &Receiver<Event>,
        render_tx: &Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> EngineSignal {
        let mut batch: Vec<EngineSignal> = Vec::new();
        for event in event.try_iter() {
            match event {
                Event::Keyboard(key) => match key {
                    KeyEvent::Char('s') => {
                        if self.menu.cursor_down(1) {
                            self.menu.output(render_tx)
                        }
                    }
                    KeyEvent::Char('w') => {
                        if self.menu.cursor_up(1) {
                            self.menu.output(render_tx)
                        }
                    }
                    KeyEvent::Char('d') => match self.menu.execute(()) {
                        Signal::Back => {
                            batch.push(EngineSignal::Scenes(crate::engine::enums::SceneSignal::Pop))
                        }
                        Signal::NewWorld => batch.push(EngineSignal::Scenes(
                            crate::engine::enums::SceneSignal::New {
                                scene: CreateWorld::new(),
                                signal: None,
                            },
                        )),
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        if batch.len() > 0 {
            return EngineSignal::Batch(batch);
        }
        return EngineSignal::None;
    }
}

fn add_load_files_to_menu(menu: &mut Menu<Option<DirEntry>, Signal>, files: &Vec<DirEntry>) {
    for (index, save) in files.iter().enumerate() {
        // Checking Extntion
        if !match save.path().extension() {
            None => "".to_string(),
            Some(ext) => match ext.to_str() {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
        }
        .eq("world")
        {
            // Log that file is not have valid extention
            continue;
        }

        menu.add(MenuItem::new(
            "Label".to_string(),
            |dir: Option<DirEntry>| load_world(dir),
        ));
    }
}

fn load_world(mut dir: Option<DirEntry>) -> Signal {
    Signal::None
}
fn new_world(_: Option<DirEntry>) -> Signal {
    Signal::None
}

fn get_saves_list(path: &Path) -> Vec<DirEntry> {
    let mut saves = Vec::<DirEntry>::new();
    if path.exists() {
        if !path.is_dir() {
            // Log that there was a problem
            if let Err(_e) =
                fs::create_dir_all(path.to_str().expect("Path failed to convert to &str!"))
            {
                // Log that there was a problem
            }
            return vec![];
        }
        let mut dir = match path.read_dir() {
            Err(_e) => {
                // Log that there was a problem
                return vec![];
            }
            Ok(dir) => dir,
        };

        #[allow(for_loops_over_fallibles)]
        for item in dir.next() {
            let item = match item {
                Ok(item) => item,
                Err(_e) => {
                    // Log That ther was a problem
                    continue;
                }
            };
            let type_data = match item.file_type() {
                Ok(data) => data,
                Err(_e) => {
                    // Log that there was a problem
                    continue;
                }
            };

            if type_data.is_dir() {
                continue;
            }

            let name = match item.file_name().to_str() {
                Some(s) => String::from(s),
                None => {
                    // Log that there was a problem
                    continue;
                }
            };
            if check_file_extention(&item) {
                saves.push(item);
            }
        }
    }
    return saves;
}

fn check_file_extention(item: &DirEntry) -> bool {
    match item.path().extension() {
        Some(ext) => match ext.to_str() {
            Some(s) => s.contains("world"),
            None => false,
        },
        None => false,
    }
}
