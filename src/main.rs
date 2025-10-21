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

#![allow(dead_code, unused)]
mod engine;
mod game;
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
