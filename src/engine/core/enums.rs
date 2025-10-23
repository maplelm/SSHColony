use crate::engine::input::Event;
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
use crate::engine::render::RenderUnitId;

use super::super::super::render::{Canvas, Layer, Object};
use super::super::Error;
use super::super::types::Position3D;
use super::traits::Scene;
use std::sync::{Arc, atomic::AtomicUsize};
use term::Terminal;
use term::color::{Background, Foreground};

pub enum Signal {
    None,
    Quit,
    CrossScene { label: String, data: Encoding },
    Scenes(SceneSignal),
    Render(RenderSignal),
    Error(Error),
    Log(String),
    Batch(Vec<Signal>),
    Sequence(Vec<Signal>),
}

pub enum Encoding {
    Json,
    Bson,
    Bincode,
}

pub enum SceneSignal {
    Pop,
    New {
        scene: Box<dyn Scene>,
        signal: Option<Box<Signal>>,
    },
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
