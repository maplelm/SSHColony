#![allow(unreachable_patterns)]

use super::{MainMenu, Settings, InGame};
use crate::engine::{Scene, Signal};
//use crate::engine::Scene::is_paused;
use std::sync::mpsc;

pub enum Game{
    MainMenu(MainMenu::<Game>),
    Settings(Settings::<Game>),
    InGame(InGame::<Game>),
}

impl Scene<Game> for Game {
    fn init(&mut self) {
        
    }
    fn is_paused(&self) -> bool {
        match self {
            Game::MainMenu(mm) => mm.is_paused(),
            Game::Settings(s) => s.is_paused(),
            Game::InGame(ig) => ig.is_paused(),
            _ => todo!()
        }
    }
    fn reset(&mut self) {
        match self {
            Game::MainMenu(s) => s.reset(),
            Game::Settings(s) => s.reset(),
            Game::InGame(s) => s.reset(),
            _ => todo!()
        }
    }
    fn resume(&mut self) {
        match self {
            Game::MainMenu(s) => s.resume(),
            Game::Settings(s) => s.resume(),
            Game::InGame(s) => s.resume(),
            _ => todo!()
        }
    }
    fn suspend(&mut self) {
        match self {
            Game::MainMenu(s) => s.suspend(),
            Game::Settings(s) => s.suspend(),
            Game::InGame(s) => s.suspend(),
            _ => todo!()
        }
    }
    fn update(
            &mut self,
            delta_time: f32,
            event: &mpsc::Receiver<crate::engine::Event>,
            render_tx: &std::sync::mpsc::Sender<crate::engine::RenderMsg>,
        ) -> Signal<Game> {
        match self {
            Game::MainMenu(s) => s.update(delta_time, event, render_tx),
            Game::Settings(s) => s.update(delta_time, event, render_tx),
            Game::InGame(s) => s.update(delta_time, event, render_tx),
            _ => todo!()
        }
    }
}