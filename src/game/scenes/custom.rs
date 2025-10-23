use crate::engine::enums::{Signal as EngineSignal, RenderSignal, SceneDataMsg};
use crate::engine::input::Event;
use crate::engine::render::Canvas;
use crate::engine::traits::Scene;
use mlua::{ Lua, Table, Function };
use std::sync::mpsc;

pub struct LuaScene {
    lua: Lua,
    table: Table,
}

impl LuaScene {
    pub fn from_file (lua: Lua, path: &str) -> Box<dyn Scene> {
        let script = std::fs::read_to_string(path).unmwrap();
        let table: Table = lua.load(&script).eval().unwrap();
        Box::new(Self{lua, table})
    }
}

impl Scene for CustomScene {
    fn init(&mut self, render_tx: &mpsc::Sender<RenderSignal>, signal: Option<SceneDataMsg>, canvas: &Canvas) -> EngineSignal {
        if let Ok(func) = self.table.get::<_, Function>("init") {
            let _res = func.call::<_, ()>((self.table.clone(),)); // Pass itself
            // What does _res hold?
            // push it to file? 
        }
        return EngineSignal::None;
    }

    fn is_init(&self) -> bool { true }
    fn update(
        &mut self,
        delta_time: f32,
        event_rx: &mpsc::Receiver<Event>
        render_tx: &mpsc::Sender<RenderSignal>,
        canvas: &Canvas,
        ) -> EngineSignal {
        // Convert the Event system into a lua table for the script
        EngineSignal::None
    }
    fn resume(&mut self, render_tx: &mpsc::Sender<RenderSignal>, canvas: &Canvas) {}
    fn suspend(&mut self, render_tx: &mpsc::Sender<RenderSignal>, canvas: &Canvas) {}
    fn is_paused(&self) -> bool {
        return false
    }
    fn reset(&mut self) {}
}
