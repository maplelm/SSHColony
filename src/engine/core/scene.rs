use crate::engine::enums::Signal as EngineSignal;
use mlua::{Function, Lua, Table};

pub struct Scene {
    lua: Lua,
    table: Table,
}

impl Scene {
    pub fn from_file(name: &str) -> Self {
        let lua: Lua = Lua::new().load(name).eval().unwrap();
        let table: Table = lua.globals();
        Self { lua, table }
    }

    pub fn init(&mut self, event: String) -> EngineSignal {
        let res = self.table.get("init").unwrap().call(event);
        if res == "Quit" {
            EngineSignal::Quit
        } else {
            // Log that the lua scipt did not return a handle able result
            EngineSignal::None
        }
    }
}
