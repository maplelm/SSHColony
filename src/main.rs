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
    /*
    let mut _in = io::stdin();
    let mut _out = io::stdout();
    let orig = term::set_raw();
    write!(&_out, "\x1b[18t");
    _out.flush();
    let mut buf: [u8; 64] = [0; 64];
    _in.read(&mut buf);
    let resp = String::from_utf8_lossy(&buf);
    let resp_string = resp.to_string();
    let resp = resp_string.strip_prefix("\x1b[8").unwrap();
    write!(&_out, "\r{}\n\r", resp);
    _out.flush();
    term::set_term(orig);
    ExitCode::SUCCESS
    */
    match engine::start(Instance::new(MainMenu::new())) {
        Ok(_) => return ExitCode::SUCCESS,
        Err(e) => {
            println!("Error While Running Game, {}", e);
            return ExitCode::FAILURE;
        }
    }
}
