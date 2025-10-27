use crate::engine::enums::{RenderSignal, SceneDataMsg, SceneSignal, Signal as EngineSignal};
use crate::engine::input::Event;
use crate::engine::render::Canvas;
use mlua::{Function, IntoLua, Lua, Table, UserData};
use std::rc::Rc;
use std::sync::mpsc;

pub struct Scene {
    module: Table,
    env: Lua
    state: Rc<Table>,
    init_complete: bool,
}

impl Scene {
    pub fn new(path: &str, state: Rc<Table>) -> Result<Self, Box<dyn std::error::Error>> {
        let lua: Lua = Lua::new();
        let script = match std::fs::read_to_string(path) {
            Err(e) => return Box::new(e),
            Ok(val) => val,
        };
        let module: Table = match lua.load(script).eval() {
            Err(e) => return Box::new(e),
            Ok(val) => val,
        };

        Ok(Self {
            env: lua,
            state,
            module,
            init_complete: false,
        })
    }
}
impl Scene {
    pub fn init(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
        signal: Option<SceneDataMsg>,
        canvas: &Canvas,
    ) -> EngineSignal {
        self.init_complete = true;
        self.module.call_function("init", ()).unwrap()
    }

    pub fn is_init(&self) -> bool {
        self.init_complete
    }
    pub fn update(
        &mut self,
        delta_time: f32,
        event_rx: &mpsc::Receiver<Event>,
        render_tx: &mpsc::Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> EngineSignal {
        // Convert the Event system into a lua table for the script
        EngineSignal::None
    }
    pub fn resume(&mut self, render_tx: &mpsc::Sender<RenderSignal>, canvas: &Canvas) {}
    pub fn suspend(&mut self, render_tx: &mpsc::Sender<RenderSignal>, canvas: &Canvas) {}
    pub fn is_paused(&self) -> bool {
        return false;
    }
    pub fn reset(&mut self) {}
}
