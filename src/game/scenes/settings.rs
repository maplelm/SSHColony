#![allow(unused)]
use crate::{
    engine::{
        self,
        enums::{RenderSignal, SceneSignal, Signal},
        input::{Event, KeyEvent},
        render::{self, Canvas, Object, RenderUnitId},
        traits::Scene,
        types::Position3D,
        ui::style::{Align, Justify},
    },
    game::Game,
};
use std::{
    marker::PhantomData,
    sync::{Arc, Weak, atomic::AtomicUsize, mpsc},
};

pub struct Settings {
    text_handle: Weak<RenderUnitId>,
    init_complete: bool,
}

impl Settings {
    pub fn new() -> Game {
        Game::Settings(Self {
            text_handle: Weak::new(),
            init_complete: false,
        })
    }
}

impl Settings {
    pub fn init(
        &mut self,
        render_tx: &mpsc::Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> Signal<Game> {
        render_tx.send(RenderSignal::Clear);
        let arc_new = Arc::<RenderUnitId>::new(RenderUnitId::Ui(AtomicUsize::new(0)));
        self.text_handle = Arc::downgrade(&arc_new);
        let obj = Object::static_text(
            Position3D { x: 1, y: 1, z: 0 },
            "Settings!".to_string(),
            Justify::Left,
            Align::Top,
            None,
            None,
            None,
            None,
            None,
        );
        render_tx.send(RenderSignal::Insert(arc_new, obj));
        self.init_complete = true;

        Signal::None
    }
    pub fn is_init(&self) -> bool {
        return self.init_complete;
    }
    pub fn is_paused(&self) -> bool {
        false
    }
    pub fn reset(&mut self) {}
    pub fn resume(&mut self, canvas: &Canvas) {}
    pub fn suspend(&mut self) {}
    pub fn update(
        &mut self,
        delta_time: f32,
        event: &mpsc::Receiver<Event>,
        render_tx: &std::sync::mpsc::Sender<RenderSignal>,
        canvas: &Canvas,
    ) -> Signal<Game> {
        let mut signals: Vec<Signal<Game>> = vec![];
        for each in event.try_iter() {
            match each {
                Event::Keyboard(k) => match k {
                    KeyEvent::Char('q') => signals.push(Signal::Scenes(SceneSignal::Pop)),
                    _ => {}
                },
                _ => {}
            }
        }
        if signals.len() == 0 {
            return Signal::None;
        } else if signals.len() == 1 {
            return signals.remove(0);
        } else {
            return Signal::Batch(signals);
        }
    }
}
