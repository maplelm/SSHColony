#![allow(unreachable_patterns)]

use super::scenes::{InGame, LoadGame, MainMenu, Settings};
use crate::engine::{
    enums::Signal,
    input::Event,
    render::{self, Canvas},
    traits::Scene,
};
//use crate::engine::Scene::is_paused;
use std::sync::mpsc;

pub enum Game {
    MainMenu(MainMenu),
    Settings(Settings),
    InGame(InGame),
    LoadGame(LoadGame),
}

impl Scene<Game> for Game {
    fn init(&mut self, render_tx: &mpsc::Sender<render::Msg>, canvas: &Canvas) -> Signal<Game> {
        match self {
            Game::MainMenu(s) => s.init(render_tx, canvas),
            Game::InGame(s) => s.init(render_tx, canvas),
            Game::Settings(s) => s.init(render_tx, canvas),
            Game::LoadGame(s) => s.init(render_tx, canvas),
        }
    }
    fn is_init(&self) -> bool {
        match self {
            Game::MainMenu(s) => s.is_init(),
            Game::InGame(s) => s.is_init(),
            Game::Settings(s) => s.is_init(),
            Game::LoadGame(s) => s.is_init(),
        }
    }
    fn is_paused(&self) -> bool {
        match self {
            Game::MainMenu(mm) => mm.is_paused(),
            Game::Settings(s) => s.is_paused(),
            Game::InGame(ig) => ig.is_paused(),
            _ => todo!(),
        }
    }
    fn reset(&mut self) {
        match self {
            Game::MainMenu(s) => s.reset(),
            Game::Settings(s) => s.reset(),
            Game::InGame(s) => s.reset(),
            Game::LoadGame(s) => s.reset(),
        }
    }
    fn resume(&mut self, render_tx: &mpsc::Sender<render::Msg>, canvas: &Canvas) {
        match self {
            Game::MainMenu(s) => s.resume(render_tx, canvas),
            Game::Settings(s) => s.resume(canvas),
            Game::InGame(s) => s.resume(canvas),
            Game::LoadGame(s) => s.resume(render_tx, canvas),
        }
    }
    fn suspend(&mut self, render_tx: &mpsc::Sender<render::Msg>) {
        match self {
            Game::MainMenu(s) => s.suspend(render_tx),
            Game::Settings(s) => s.suspend(),
            Game::InGame(s) => s.suspend(),
            Game::LoadGame(s) => s.suspend(),
        }
    }
    fn update(
        &mut self,
        delta_time: f32,
        event: &mpsc::Receiver<Event>,
        render_tx: &std::sync::mpsc::Sender<render::Msg>,
        canvas: &Canvas,
    ) -> Signal<Game> {
        match self {
            Game::MainMenu(s) => s.update(delta_time, event, render_tx, canvas),
            Game::Settings(s) => s.update(delta_time, event, render_tx, canvas),
            Game::InGame(s) => s.update(delta_time, event, render_tx, canvas),
            Game::LoadGame(s) => s.update(event, render_tx, canvas),
        }
    }
}
