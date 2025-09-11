pub mod engine;
pub mod log;
pub mod scenes;

use std::process::ExitCode;
use crate::scenes::MainMenu;

fn main() -> ExitCode {
    let _ = engine::run::<scenes::Game>(crate::engine::Instance::new(MainMenu::new()));
    ExitCode::SUCCESS
}
