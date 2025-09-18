#![allow(dead_code, unused)]
mod engine;
mod game;
mod log;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
    process::ExitCode
};

use crate::engine::{
    Instance,
    render,
    render::Canvas,
    types::{Position, Position3D, Store},
};
use game::MainMenu;
use ron;

fn main() -> ExitCode {
    let _ = engine::run(Instance::new(
        MainMenu::new(),
        Canvas {
            width: 100,
            height: 50,
        },
    ));
    ExitCode::SUCCESS
}
