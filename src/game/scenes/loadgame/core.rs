//#![deny(unused)]

use crate::engine::enums::RenderSignal;
use crate::engine::enums::Signal as EngineSignal;
use crate::engine::render::clear as render_clear;
use crate::engine::ui::BorderSprite as Bsprite;
use crate::engine::ui::MenuItem;

use super::super::super::types::World;
use crate::engine::input::{Event, KeyEvent};
use crate::engine::{
    render::Canvas,
    ui::{
        Border, Menu, Padding,
        style::{Justify, Measure, Origin},
    },
};
use crate::game::Game;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

enum Signal {
    None,
    NewWorld,
    LoadWorld(String),
    WorldData(World),
    Back
}

pub struct LoadGame {
    menu: Menu<Option<DirEntry>, Signal>,
    is_init: bool,
}

impl LoadGame {
    pub fn new() -> Game {
        Game::LoadGame(Self {
            menu: Menu::<Option<DirEntry>, Signal>::new(
                0,
                0,
                Some(Measure::Percent(100)),
                Some(Measure::Percent(100)),
                Origin::TopLeft,
                Justify::Left,
                Some(Border::from(
                    Bsprite::String("#%".to_string()),
                    Padding::square(2),
                )),
                vec![],
                None,
                None,
            ),
            is_init: false,
        })
    }
    pub fn init(
        &mut self,
        render_tx: &Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> EngineSignal<Game> {
        if let Err(_e) = render_clear(render_tx) {
            // log that there was a problem
        }
        let save_dir: &str = "./saves/";
        let save_path: &Path = Path::new(save_dir);
        let mut saves: Vec<DirEntry> = get_saves_list(save_path);
        add_load_files_to_menu(&mut self.menu, &saves);
        self.menu
            .add(MenuItem::new("New World".to_string(), new_world));
        self.menu
            .add(MenuItem::new("Back".to_string(), |_| {Signal::Back}));

        self.menu.output(render_tx);
        EngineSignal::None
    }

    pub fn is_init(&self) -> bool {
        true
    }

    pub fn is_paused(&self) -> bool {
        false
    }

    pub fn reset(&mut self) {}

    pub fn resume(&mut self, render_tx: &Sender<RenderSignal>, canvas: &Canvas) {
        render_tx.send(RenderSignal::Clear);
        self.menu.output(render_tx);
    }

    pub fn suspend(&mut self, render_tx: &Sender<RenderSignal>) {
        render_tx.send(RenderSignal::Clear);
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        event: &Receiver<Event>,
        render_tx: &Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> EngineSignal<Game> {
        let mut batch: Vec<EngineSignal<Game>> = Vec::new();
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
                    KeyEvent::Char('d') => {
                        match self.menu.execute(None) {
                            Signal::Back => batch.push(EngineSignal::Scenes(crate::engine::enums::SceneSignal::Pop)),
                            _ => {}
                        }
                    }
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

