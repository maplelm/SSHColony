#![deny(unused)]

use crate::{
    engine::{
        self,
        enums::Signal,
        input::{Event, KeyEvent},
        render::{self, Canvas},
        types::Position,
        ui::{
            Border, BorderSprite, Menu, MenuItem, Padding,
            style::{Justify, Origin},
        },
    },
    game::{Game, InGame},
};
use std::{fs, path::Path, sync::mpsc};

#[allow(unused)]
enum SceneSignals {
    NewWorld,
    LoadWorld,
}

pub struct LoadGame {
    saves_menu: Menu<LoadGame, SceneSignals>,
    init_complete: bool,
}

impl LoadGame {
    pub fn new() -> Game {
        Game::LoadGame(Self {
            saves_menu: Menu::new(
                0,
                0,
                Some(engine::ui::style::Measure::Percent(100)),
                Some(engine::ui::style::Measure::Percent(100)),
                Origin::TopLeft,
                Justify::Left,
                Some(Border::from(
                    BorderSprite::String(String::from("#=~")),
                    Padding::square(3),
                )),
                vec![],
            ),
            init_complete: false,
        })
    }

    pub fn init(&mut self, render_tx: &mpsc::Sender<render::Msg>, canvas: &Canvas) -> Signal<Game> {
        if let Err(_e) = render::clear(render_tx) {
            // log that there was a problem
        }
        let save_dir: &str = "./saves/";
        let save_path: &Path = Path::new(save_dir);
        let mut saves: Vec<String> = vec![];

        // Check if there are any saved worlds in ./saves/

        if save_path.exists() {
            if !save_path.is_dir() {
                panic!("the save dir {} has to be a directory not a file", save_dir)
            }
            let d = fs::read_dir(save_dir);
            if let Err(e) = d {
                panic!("failed to read directory {}, {}", save_dir, e)
            }
            let mut d = d.unwrap();
            // Process each file

            loop {
                let dir_object = d.next();
                if dir_object.is_none() {
                    break;
                }
                let dir_object = dir_object.unwrap();
                if let Err(e) = dir_object {
                    panic!("Failed to Get Item in save directory {}, {}", save_dir, e);
                }
                let dir_object = dir_object.unwrap();
                if let Ok(t_data) = dir_object.file_type()
                    && t_data.is_dir()
                {
                    continue;
                }
                let os_string = dir_object.file_name();
                let name = os_string
                    .to_str()
                    .expect("Failed to get file name while parsing save file directory");
                let parts = name.split(".");
                let count = name.split(".").count();
                let mut name = String::new();
                for (i, each) in parts.enumerate() {
                    if i != count - 1 {
                        name.push_str(each);
                    }
                }
                let name = name.replace("_", " ");
                saves.push(name);
            }
        } else {
            if let Err(_e) = fs::create_dir_all(save_dir) {
                // Log that there is a problem
            }
        }

        for (i, each) in saves.iter().enumerate() {
            self.saves_menu.add(MenuItem::new(
                String::from(&usize::to_string(&(i + 1))) + ". " + each,
                |_g: &LoadGame| -> Option<SceneSignals> { None },
            ))
        }

        self.saves_menu.add(MenuItem {
            label: String::from("Create New World"),
            action: |_g: &LoadGame| -> Option<SceneSignals> { Some(SceneSignals::NewWorld) },
        });

        //build the create world menu just because it will be fast and easier to do here
        // If Yes Ask if load world or new world
        // if no auto new world
        // if new world ask user about world settings

        // load or generate world
        // open world in the InGame Scene
        if let Err(_e) = render_tx.send(render::Msg::InsertText {
            pos: engine::types::Position::<usize> {
                x: self.saves_menu.x(),
                y: self.saves_menu.y(),
            },
            text: self.saves_menu.output(canvas),
            prefix: None,
            suffix: None,
        }) {
            // Log that something whent wrong
        }

        Signal::None
    }

    pub fn is_init(&self) -> bool {
        self.init_complete
    }

    pub fn _is_paused(&self) -> bool {
        false
    }

    pub fn reset(&mut self) {}

    pub fn suspend(&mut self) {}

    pub fn resume(&mut self, render_tx: &mpsc::Sender<render::Msg>, canvas: &Canvas) {
        if let Err(_e) = render_tx.send(render::Msg::InsertText {
            pos: Position {
                x: self.saves_menu.x(),
                y: self.saves_menu.y(),
            },
            text: self.saves_menu.output(canvas),
            prefix: None,
            suffix: None,
        }) {
            // Log that there is a problem
        }
    }

    pub fn update(
        &mut self,
        event: &mpsc::Receiver<Event>,
        render_tx: &mpsc::Sender<render::Msg>,
        _canvas: &Canvas,
    ) -> Signal<Game> {
        for e in event.try_iter() {
            match e {
                Event::Keyboard(key) => match key {
                    KeyEvent::Char('q') => return Signal::Quit,
                    KeyEvent::Char('w') => {
                        let pp = self.saves_menu.cursor_pos();
                        if self.saves_menu.cursor_up(1) {
                            let _ = render_tx.send(render::Msg::Batch(vec![
                                render::Msg::Remove(pp),
                                render::Msg::Insert(
                                    self.saves_menu.cursor_pos(),
                                    self.saves_menu.marker_object(),
                                ),
                            ]));
                        }
                    }
                    KeyEvent::Char('s') => {
                        let pp = self.saves_menu.cursor_pos();
                        if self.saves_menu.cursor_down(1) {
                            let _ = render_tx.send(render::Msg::Batch(vec![
                                render::Msg::Remove(pp),
                                render::Msg::Insert(
                                    self.saves_menu.cursor_pos(),
                                    self.saves_menu.marker_object(),
                                ),
                            ]));
                        }
                    }
                    KeyEvent::Char('d') => {
                        if let Some(sig) = self.saves_menu.execute(self) {
                            match sig {
                                SceneSignals::LoadWorld => {}
                                SceneSignals::NewWorld => match InGame::new() {
                                    Err(e) => return Signal::Error(e),
                                    Ok(s) => return Signal::NewScene(s),
                                },
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Signal::None
    }
}
