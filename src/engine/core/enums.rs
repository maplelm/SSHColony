use crate::engine::render::RenderUnitId;

use super::super::super::render::{Layer, Object};
use super::super::Error;
use super::super::types::Position3D;
use super::traits::Scene;
use std::sync::{Arc, atomic::AtomicUsize};
use term::Terminal;
use term::color::{Background, Foreground};

pub enum Signal<T: Scene<T>> {
    None,
    Quit,
    Scenes(SceneSignal<T>),
    Render(RenderSignal),
    Error(Error),
    Log(String),
    Batch(Vec<Signal<T>>),
    Sequence(Vec<Signal<T>>),
}

pub enum SceneSignal<T: Scene<T>> {
    Pop,
    New(T),
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
