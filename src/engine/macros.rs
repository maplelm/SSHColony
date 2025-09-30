#![allow(unused, unused_variables)]
use std::sync::mpsc::Sender;
use crate::engine::ui::Output;
use crate::engine::render::Msg;

#[macro_export]
macro_rules! menu_item_scene_push {
    ($scene:ty, $variant:expr, $n:ident) => {
        #[derive(Debug)]
        struct $n {
            n: String,
            t: Sender<$scene>
        } 

        impl $n {
            pub fn new(n: String, t: Sender<$scene>) -> Self {
                Self {
                    n: n,
                    t: t
                }
            }
        }

        impl MenuItem for $n {
            fn label(&self) -> &str {
                &self.n
            }
            fn execute(&mut self) -> bool {
                self.t.send($variant);
                false
            }
            fn output(&mut self) -> Output {
                Output::None
            }
        }
    };
}

#[macro_export]
macro_rules! new_scene_struct {
    ($scene_type:ty, $name:ident,$init:expr,$is_init:expr, $is_paused:expr, $update:expr, $resume:expr, $suspend:expr, $reset:expr, $args:ident{$($arg:ident : $t:ty), * $(,)?}) => {

    #[allow(unused, unused_variables)]
    struct $name {
        $($arg: $t),*
    }

    impl $name {
        #[allow(unused, unused_variables)]
        pub fn init(&mut self, render_tx: Sender<crate::engine::render::Msg>, canvas: &Canvas) -> Signal<$scene_type> {
            $init
        }
        pub fn is_init(self) -> bool {
            $is_init
        }
        #[allow(unused, unused_variables)]
        pub fn update(&mut self, delta_time: f32, event_tx: &std::sync::mpsc::Receiver<crate::engine::input::Event>, render_tx: &std::sync::mpsc::Sender<crate::engine::render::Msg>, canvas: &crate::engine::render::Canvas) -> Signal<$scene_type> {
            $update
        }
        #[allow(unused, unused_variables)]
        pub fn suspend(&mut self, render_tx: Sender<crate::engine::render::Msg>) {
            $suspend
        }
        #[allow(unused, unused_variables)]
        pub fn resume(&mut self, render_tx: std::sync::mpsc::Sender<crate::engine::render::Msg>, canvas: &Canvas) {
            $resume
        }
        #[allow(unused, unused_variables)]
        pub fn is_paused(&self) -> bool {
            $is_paused
        }
        #[allow(unused, unused_variables)]
        pub fn reset(&mut self) {
            $reset
        }
    }
        
    };
}