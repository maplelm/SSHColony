use super::super::Error;
use super::traits::Scene;
use term::Terminal;

//#[derive(Clone)]
pub enum Signal<T: Scene<T>> {
    None,
    Quit,
    PopScene,
    NewScene(T),
    TerminalState(Terminal),
    Batch(Vec<Signal<T>>),
    Sequence(Vec<Signal<T>>),
    Error(Error),
}

