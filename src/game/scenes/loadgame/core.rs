//#![deny(unused)]

use crate::engine::ui::MenuItem;
use crate::engine::ui::BorderSprite as Bsprite;
use crate::engine::enums::Signal as EngineSignal;
use crate::engine::render::clear as render_clear;

use crate::engine::{
    ui::{Border, Menu, Padding, style::{Justify, Measure, Origin}},
    render::{Msg, Canvas}
};
use crate::game::Game;
use std::fs::{self, DirEntry};
use std::sync::mpsc::{Sender, Receiver};
use crate::engine::input::Event;
use std::path::Path;
use super::super::super::types::World;

enum Signal {
    NewWorld,
    LoadWorld(String),
    WorldData(World)
}


#[derive(Copy)]
pub struct LoadGame {
    menu: Menu,
    is_init: bool,
}

impl LoadGame {
    pub fn new() -> Game {
        Game::LoadGame(Self{
            menu: Menu::new(
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
            ),
            is_init: false
        })
    }
    pub fn init(&mut self, render_tx: &Sender<Msg>, canvas: &Canvas) -> EngineSignal<Game> {
        if let Err(_e) = render_clear(render_tx) {
            // log that there was a problem
        } 
        let save_dir: &str = "./saves/";
        let save_path: &Path = Path::new(save_dir);
        let mut saves: Vec<DirEntry> = get_saves_list(save_path);
        add_load_files_to_menu(&mut self.menu, &saves);
        self.menu.add(MenuItem::new(
            "New World".to_string(),
            new_world
        ));
        EngineSignal::None
    }

    pub fn is_init(&self) -> bool {
        true
    }

    pub fn is_paused(&self) -> bool {
        false
    }

    pub fn reset(&mut self) {}

    pub fn resume(&mut self, render_tx: &Sender<Msg>, canvas: &Canvas) {
        
    }

    pub fn suspend(&mut self, render_tx: &Sender<Msg>) {}

    pub fn update(&mut self, delta_time: f32, event: &Receiver<Event>, render_tx: &Sender<Msg>, canvas: &Canvas) -> EngineSignal<Game> { EngineSignal::None}
}

fn add_load_files_to_menu(menu: &mut Menu<Option<DirEntry>, Signal>, files: &Vec<DirEntry>) {
    for (index, save) in files.iter().enumerate() {
        // Checking Extntion
        if !match save.path().extension() {
            None => "".to_string(),
            Some(ext) => match ext.to_str() {
                Some(s) => s.to_string(),
                None => "".to_string()
            }
        }.eq("world") {
            // Log that file is not have valid extention
            continue;
        }

        menu.add(MenuItem::new(
            "Label".to_string(),
            |dir: Option<DirEntry>| load_world(dir)
        ));
    }
}

fn load_world(mut dir: Option<DirEntry>) -> Option<Signal> { None }
fn new_world(_: Option<DirEntry>) -> Option<Signal> { None }

fn get_saves_list(path: &Path) -> Vec<DirEntry> {
    let mut saves = Vec::<DirEntry>::new();
    if path.exists() {
        if !path.is_dir() {
            // Log that there was a problem
            if let Err(_e) = fs::create_dir_all(path.to_str().expect("Path failed to convert to &str!")) {
                // Log that there was a problem
            }
            return vec![]
        }
        let mut dir = match path.read_dir() {
            Err(_e) => {
                // Log that there was a problem
                return vec![]
            }
            Ok(dir) => dir
        };

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
    match item.path().extension(){
        Some(ext)=> {
            match ext.to_str() {
                Some(s) => s.contains("world"),
                None => false
            }
        }
        None => false
    }
}