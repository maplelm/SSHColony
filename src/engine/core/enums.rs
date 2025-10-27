/*
Copyright 2025 Luke Maple

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
you may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
use crate::engine::input::Event;
use crate::engine::render::RenderUnitId;

use super::super::super::render::{Canvas, Layer, Object};
use super::super::Error;
use super::super::types::Position3D;
use super::traits::Scene;
use mlua::{FromLua, String as LuaString, Table, UserData};
use my_term::Terminal;
use my_term::color::{Background, Foreground};
use ron::value;
use std::any::{Any, type_name_of_val};
use std::sync::{Arc, atomic::AtomicUsize};

pub enum Signal {
    None,
    Quit,
    SceneUp,
    SceneDown,
    Scenes(SceneSignal),
    Render(RenderSignal),
    Error(Error),
    Log(String),
    Batch(Vec<Signal>),
    Sequence(Vec<Signal>),
}

impl UserData for Signal {}
impl FromLua for Signal {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => {
                // Need to log this here
                Err(mlua::Error::FromLuaConversionError {
                    from: "UserData",
                    to: "Signal".into(),
                    message: Some(format!("a userdata type was returned {:?}", ud)),
                })
            }
            mlua::Value::String(s) => match s {
                "None" => Ok(Self::None),
                "Quit" => Ok(Self::Quit),
                "SceneUp" => Ok(Self::SceneUp),
                "SceneDown" => Ok(Self::SceneDown),
                other => Err(mlua::Error::FromLuaConversionError {
                    from: "string",
                    to: "Signal".into(),
                    message: Some(format!("unknown engine signal {}", other)),
                }),
            },
            mlua::Value::Table(t) => match t.get::<String>("type")? {
                "Scenes" => Ok(Self::Scenes(t.get::<SceneSignal>("value")?)),
                "Render" => Ok(Self::Render(t.get::<RenderSignal>("value")?)),
                "Error" => Ok(Self::Error(t.get::<Error>("value")?)),
                "Batch" => {
                    let list: mlua::Table = t.get::<mlua::Table>("value")?;
                    let mut v = Vec::new();
                    for pair in list.sequence_values::<Signal>() {
                        v.push(pair?);
                    }
                    Ok(Self::Batch(v))
                }
                "Sequence" => {
                    let list: Table = t.get("value")?;
                    let mut v = Vec::new();
                    for pair in list.sequence_values::<Signal>() {
                        v.push(pair?);
                    }
                    Ok(Self::Sequence(v))
                }
                other => Err(mlua::Error::FromLuaConversionError {
                    from: other,
                    to: "Signal".into(),
                    message: Some(format!("invalid Signal Type {:?}", other)),
                }),
            },
            mlua::Value::Error(e) => Ok(Self::Error(e)),
            mlua::Value::Nil => Ok(Self::None),
            other => Err(mlua::Error::FromLuaConversionError {
                from: type_name_of_val(other),
                to: "Signal".into(),
                message: Some(format!("invalid signal value {:?}", other)),
            }),
        }
    }
}

pub enum SceneSignal {
    Pop,
    New(String),
}

impl FromLua for SceneSignal {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Table(t) => match t.get::<String>("type")? {
                "Pop" => Ok(Self::Pop),
                "New" => Ok(Self::New(t.get::<String>("value")?)),
                other => Err(mlua::Error::FromLuaConversionError {
                    from: "Table",
                    to: "Signal".into(),
                    message: Some(format!("invalid Scene Signal Data type {:?}", other)),
                }),
            },
            other => Err(mlua::Error::FromLuaConversionError {
                from: type_name_of_val(other),
                to: "SceneSignal".into(),
                message: Some(format!("invalid data for SceneSignal {:?}", other)),
            }),
        }
    }
}

impl UserData for SceneSignal {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field("Pop", Self::Pop);
    }
}

pub enum RenderSignal {
    Insert(Arc<RenderUnitId>, Object),
    Remove(Arc<RenderUnitId>),
    Move(Arc<RenderUnitId>, Position3D<i32>),
    MoveLayer(Arc<RenderUnitId>, Layer),
    TermSizeChange(u32, u32),
    Foreground(Foreground),
    Background(Background),
    MoveCamera(Position3D<i32>),
    SetCamera(Position3D<i32>),
    Update(Arc<RenderUnitId>, Object),
    Redraw,
    Clear,
    Batch(Vec<RenderSignal>),
    Sequence(Vec<RenderSignal>),
}

impl FromLua for RenderSignal {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::String(s) => match s {
                "Redraw" => Ok(Self::Redraw),
                "Clear" => Ok(Self::Clear),
                other => Err(mlua::Error::FromLuaConversionError {
                    from: "String",
                    to: "RenderSignal".into(),
                    message: Some(format!("invalid data for RenderSignal {:?}", other)),
                }),
            },
            mlua::Value::Table(t) => match t.get::<String>("type")? {
                "Insert" => {}
                "Remove" => {}
                "Move" => {}
                "MoveLayer" => {}
                "TermSizeChange" => {}
                "Foreground" => {}
                "Background" => {}
                "MoveCamera" => {}
                "SetCamera" => {}
                "Update" => {}
                "Batch" => {}
                "Sequence" => {}
                other => Err(mlua::Error::FromLuaConversionError {
                    from: "String",
                    to: "RenderSignal".into(),
                    message: Some(format!("invalid data for RenderSignal {:?}", other)),
                }),
            },
            other => Err(mlua::Error::FromLuaConversionError {
                from: "String",
                to: "RenderSignal".into(),
                message: Some(format!("invalid data for RenderSignal {:?}", other)),
            }),
        }
    }
}

impl RenderSignal {
    // Marking as test as I don't want to be checking Signals like this for any reason other then
    // testing
    #[cfg(test)]
    pub fn as_str(&mut self, canvas: &Canvas) -> Option<&str> {
        match self {
            RenderSignal::Insert(_, obj) => Some(obj.as_str(canvas)),
            RenderSignal::Update(_, obj) => Some(obj.as_str(canvas)),
            _ => None,
        }
    }
}

////////////////
///  Macros  ///
////////////////

#[macro_export]
macro_rules! pop_scene {
    () => {
        Siganl::Scene(SceneSignal::Pop)
    };
}

#[macro_export]
macro_rules! new_scene {
    ($name:ty) => {
        Signal::Scenes(SceneSignal::New($name))
    };
}
