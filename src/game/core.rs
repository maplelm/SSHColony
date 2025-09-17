#![allow(unreachable_patterns)]

use super::{InGame, MainMenu, Settings};
use crate::engine::{Scene, Signal, render};
//use crate::engine::Scene::is_paused;
use std::sync::mpsc;

pub enum Game {
    MainMenu(MainMenu),
    Settings(Settings),
    InGame(InGame),
}

impl Scene<Game> for Game {
    fn init(&mut self, render_tx: &mpsc::Sender<render::Msg>) {
        match self {
            Game::MainMenu(s) => s.init(render_tx),
            Game::InGame(s) => s.init(render_tx),
            Game::Settings(s) => s.init(render_tx),
        }
    }
    fn is_init(&self) -> bool {
        match self {
            Game::MainMenu(s) => s.is_init(),
            Game::InGame(s) => s.is_init(),
            Game::Settings(s) => s.is_init(),
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
            _ => todo!(),
        }
    }
    fn resume(&mut self, render_tx: &mpsc::Sender<render::Msg>) {
        match self {
            Game::MainMenu(s) => s.resume(render_tx),
            Game::Settings(s) => s.resume(),
            Game::InGame(s) => s.resume(),
            _ => todo!(),
        }
    }
    fn suspend(&mut self, render_tx: &mpsc::Sender<render::Msg>) {
        match self {
            Game::MainMenu(s) => s.suspend(render_tx),
            Game::Settings(s) => s.suspend(),
            Game::InGame(s) => s.suspend(),
            _ => todo!(),
        }
    }
    fn update(
        &mut self,
        delta_time: f32,
        event: &mpsc::Receiver<crate::engine::Event>,
        render_tx: &std::sync::mpsc::Sender<render::Msg>,
    ) -> Signal<Game> {
        match self {
            Game::MainMenu(s) => s.update(delta_time, event, render_tx),
            Game::Settings(s) => s.update(delta_time, event, render_tx),
            Game::InGame(s) => s.update(delta_time, event, render_tx),
            _ => todo!(),
        }
    }
}

pub type MaterialID = u32;
//pub type SpriteID = u32;
pub type EntityID = u32;
