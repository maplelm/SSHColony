use crate::engine::enums::{RenderSignal, SceneDataMsg, SceneSignal, Signal as EngineSignal};
use crate::engine::input::Event;
use crate::engine::render::Canvas;
use mlua::{Function, IntoLua, Lua, Table, UserData};
use std::sync::mpsc;

pub struct Scene {
    lua: Lua,          // Lua Context
    scene_data: Table, // Global variables
}

impl UserData for Scene {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("AddScene", |lua: &Lua, v: String| {
            Ok(Self::from_file(*lua, &v))
        });
        methods.add_function("PopScene", |lua: &Lua, v: ()| {
            Ok(EngineSignal::Scenes(SceneSignal::Pop))
        });
    }
}

impl Scene {
    pub fn from_file(lua: Lua, path: &str) -> Self {
        let script = std::fs::read_to_string(path).unwrap();
        let scene_data: Table = lua.load(&script).eval().unwrap();
        Self { lua, scene_data }
    }
}
impl Scene {
    pub fn init(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
        signal: Option<SceneDataMsg>,
        canvas: &Canvas,
    ) -> EngineSignal {
        return EngineSignal::None;
    }

    pub fn is_init(&self) -> bool {
        true
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
