use super::super::core::traits::Scene;
use super::super::{Context, consts::DEFAULT_CANVAS, render::Canvas};
use term::{Terminal, term_size};

pub struct Instance<T: Scene<T>> {
    pub ctx: Context,
    pub term_orig: Terminal,
    pub game_state: Vec<T>,
    pub canvas: Canvas,
}

impl<T: Scene<T> + Copy> Default for Instance<T> {
    fn default() -> Self {
        let mut canvas = DEFAULT_CANVAS;
        if let Some(size) = term_size() {
            canvas.width = size.0 as usize;
            canvas.height = size.1 as usize;
        }
        Self {
            ctx: Context::new(),
            term_orig: Terminal::default(),
            game_state: vec![],
            canvas: canvas,
        }
    }
}

impl<T: Scene<T>> Drop for Instance<T> {
    fn drop(&mut self) {}
}

impl<T: Scene<T>> Instance<T> {
    pub fn new(init_scene: T) -> Self {
        let mut canvas = DEFAULT_CANVAS;
        if let Some(size) = term_size() {
            canvas.width = size.0 as usize;
            canvas.height = size.1 as usize;
        }
        Self {
            ctx: Context::new(),
            term_orig: Terminal::default(),
            game_state: vec![init_scene],
            canvas: canvas,
        }
    }

    pub fn add_scene(&mut self, s: T) {
        self.game_state.push(s);
    }
}
