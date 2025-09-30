use super::super::super::render::Object;
use super::super::Error;
use super::super::types::Position;
use super::traits::Scene;
use term::Terminal;

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
    Insert(Position<usize>, Object),
    InsertRange {
        start: Position<usize>,
        text: String,
        foreground: Option<term::color::Foreground>,
        background: Option<term::color::Background>,
    },
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

