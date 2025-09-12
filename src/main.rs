mod engine;
mod game;
mod log;

use engine::{Instance, render::Canvas};
use game::MainMenu;
use std::process::ExitCode;

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
