#![allow(dead_code, unused)]
mod engine;
mod game;
mod log;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    process::ExitCode,
    time::{Duration, Instant},
};

use crate::engine::{
    Instance, render,
    render::Canvas,
    types::{Position, Position3D, Store},
};
use game::MainMenu;
use ron;

fn main() -> ExitCode {
    match engine::start(Instance::new(MainMenu::new())) {
        Ok(_) => return ExitCode::SUCCESS,
        Err(e) => {
            println!("Error While Running Game, {}", e);
            return ExitCode::FAILURE;
        }
    }
}
