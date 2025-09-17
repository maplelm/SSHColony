use super::world::World;
use crate::engine::{self, Scene, render};
use crate::game::Game;
use std::{marker::PhantomData, sync::mpsc};
pub struct InGame {
    world: World,
    init_complete: bool,
    is_paused: bool,
}

const DEFAULT_WORLD_X: usize = 50;
const DEFAULT_WORLD_Y: usize = 50;
const DEFAULT_WORLD_Z: usize = 1;
const DEFAULT_WORLD_TEMP: f32 = 25.0; // Celius
const DEFAULT_WORLD_HEIGHT: f32 = 1.0;
const DEFAULT_WORLD_SEA_LEVEL: f32 = 0.0;

impl InGame {
    pub fn new() -> Game {
        Game::InGame(Self {
            // I eventually want to change this to consult the settings brought into the program
            world: World::new(
                DEFAULT_WORLD_X,
                DEFAULT_WORLD_Y,
                DEFAULT_WORLD_Z,
                DEFAULT_WORLD_TEMP,
                DEFAULT_WORLD_HEIGHT,
                DEFAULT_WORLD_SEA_LEVEL,
                "./data/materials/",
                "./data/entities/",
                "./data/sprites/",
            ),
            init_complete: false,
            is_paused: false,
        })
    }
}

impl InGame {
    pub fn init(&mut self, _render_tx: &mpsc::Sender<render::Msg>) {
        let _ = self.world.generate(None);
        self.init_complete = true;
    }
    pub fn is_init(&self) -> bool {
        self.init_complete
    }
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
    pub fn reset(&mut self) {}
    pub fn resume(&mut self) {}
    pub fn suspend(&mut self) {}
    pub fn update(
        &mut self,
        delta_time: f32,
        event: &mpsc::Receiver<engine::Event>,
        render_tx: &std::sync::mpsc::Sender<render::Msg>,
    ) -> engine::Signal<Game> {
        engine::Signal::None
    }
}
