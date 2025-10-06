#![allow(unreachable_patterns)]
#![deny(unused)]

use super::scenes::{GenerateWorld, InGame, LoadGame, MainMenu, Settings};
use crate::engine::{
    enums::{RenderSignal, Signal},
    input::Event,
    render::Canvas,
    traits::Scene,
};
//use crate::engine::Scene::is_paused;
use std::sync::mpsc::{Receiver, Sender};

pub enum Game {
    MainMenu(MainMenu),
    GenerateWorld(GenerateWorld),
    Settings(Settings),
    InGame(InGame),
    LoadGame(LoadGame),
}

impl Scene<Game> for Game {
    fn init(&mut self, render_tx: &Sender<RenderSignal>, canvas: &Canvas) -> Signal<Game> {
        match self {
            Game::MainMenu(s) => s.init(render_tx, canvas),
            Game::InGame(s) => s.init(render_tx, canvas),
            Game::Settings(s) => s.init(render_tx, canvas),
            Game::LoadGame(s) => s.init(render_tx, canvas),
            Game::GenerateWorld(s) => s.init(render_tx, canvas),
        }
    }
    fn is_init(&self) -> bool {
        match self {
            Game::MainMenu(s) => s.is_init(),
            Game::InGame(s) => s.is_init(),
            Game::Settings(s) => s.is_init(),
            Game::LoadGame(s) => s.is_init(),
            Game::GenerateWorld(s) => s.is_init(),
        }
    }
    fn is_paused(&self) -> bool {
        match self {
            Game::MainMenu(s) => s.is_paused(),
            Game::Settings(s) => s.is_paused(),
            Game::InGame(s) => s.is_paused(),
            Game::GenerateWorld(s) => s.is_paused(),
            Game::LoadGame(s) => s.is_paused(),
        }
    }
    fn reset(&mut self) {
        match self {
            Game::MainMenu(s) => s.reset(),
            Game::Settings(s) => s.reset(),
            Game::InGame(s) => s.reset(),
            Game::LoadGame(s) => s.reset(),
            Game::GenerateWorld(s) => s.reset(),
        }
    }
    fn resume(&mut self, render_tx: &Sender<RenderSignal>, canvas: &Canvas) {
        match self {
            Game::MainMenu(s) => s.resume(render_tx, canvas),
            Game::Settings(s) => s.resume(canvas),
            Game::InGame(s) => s.resume(canvas),
            Game::LoadGame(s) => s.resume(render_tx, canvas),
            Game::GenerateWorld(s) => s.resume(redner_tx, canvas),
        }
    }
    fn suspend(&mut self, render_tx: &Sender<RenderSignal>) {
        match self {
            Game::MainMenu(s) => s.suspend(render_tx),
            Game::Settings(s) => s.suspend(),
            Game::InGame(s) => s.suspend(),
            Game::LoadGame(s) => s.suspend(render_tx),
            Game::GenerateWorld(s) => s.suspend(render_tx),
        }
    }
    fn update(
        &mut self,
        delta_time: f32,
        event: &Receiver<Event>,
        render_tx: &std::sync::mpsc::Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> Signal<Game> {
        match self {
            Game::MainMenu(s) => s.update(delta_time, event, render_tx, canvas),
            Game::Settings(s) => s.update(delta_time, event, render_tx, canvas),
            Game::InGame(s) => s.update(delta_time, event, render_tx, canvas),
            Game::LoadGame(s) => s.update(delta_time, event, render_tx, canvas),
            Game::GenerateWorld(s) => s.update(delta_time, event, render_tx, canvas),
        }
    }
}
