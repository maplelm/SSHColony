pub mod engine;
pub mod log;
pub mod scenes;

use engine::render::Canvas;
use std::process::ExitCode;
use crate::scenes::MainMenu;

fn main() -> ExitCode {
    let _ = engine::run(
        crate::engine::Instance::new(
            MainMenu::new(),
            Canvas{
                width: 100,
                height: 50
            }
        )
    );
    ExitCode::SUCCESS
}
