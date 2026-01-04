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
use crate::{
    game::{MainMenu},
    engine::{
        enums::{RenderSignal, SceneInitSignals, SceneSignal, Signal as EngineSignal},
        consts::DEFAULT_CANVAS,
        render::{render_thread, Canvas},
        traits::Scene,
        core::{audio_thread,event_thread},
        input::Event,
        types::{Instance, InstanceConfig},
        AudioMsg,
        Context,
        Error,
    },
};
use std::{
    ops::Deref,
    thread::{JoinHandle, spawn},
    io::{Read, stdin},
    sync::{Arc, mpsc},
    time::{Duration, Instant},
};

pub fn start(config: InstanceConfig) -> Result<(), Error> {
    /////////////////////////////////////////////////////
    // Setting up MPSC Channels for the 3 Main Threads //
    /////////////////////////////////////////////////////
    /// render_tx is used for mainly the game thread to send updates the the render threads.
    /// render_rx is given to the render thread to listen for signals passed by other threds.
    /// audio_tx is used for main the game thread to send audio change signals to the audio 
    ///     processing thread.
    /// audio_rx is given to the audio thread to listen for singals from the other threds.
    /// event_tx is actually given to the event handling thread so it can channel user input 
    ///     back to the main thread to handle as it comes in.
    /// event_rx is given to the main thread and used in the active scene to handle user input.
    let (render_tx, render_rx) = mpsc::channel();
    let (audio_tx, audio_rx) = mpsc::channel();
    let (event_tx, event_rx) = mpsc::channel();

    ////////////////////////////////////
    // Setting up the instance object //
    ////////////////////////////////////
    /// the ins object is esentually the global program state but it should be exclusive to the main logic thread.
    /// currently struggling with network data flow and if that should be on a new thread and if it does go to a new thread it will have to be
    /// removed from the instance object as that object should not be shared between threads.
    let mut ins = Instance::new(config, render_tx.clone(), event_rx);

    ////////////////////////////////////////////////////////////////////////////
    // Setting up the terminal in raw state and remembering original settings //
    ////////////////////////////////////////////////////////////////////////////
    ins.term_orig = my_term::set_raw();
    ins.term_orig.toggle_alt_buffer();
    ins.term_orig.toggle_cursor_visable();

    ////////////////////////////////////////////////////////////////////////////
    // Starting Seperate processing threads: Audio, Rendering, Event Handling //
    ////////////////////////////////////////////////////////////////////////////
    let audio_handle = start_audio_thread(ins.ctx.child(), audio_rx);
    let event_handle = start_event_thread(ins.ctx.child(), event_tx.clone());
    let render_handle = start_render_thread(
        ins.ctx.child(),
        ins.canvas.clone(),
        event_tx.clone(),
        render_rx,
        ins.logger.clone(),
    );

    /////////////////////////////////////////////////////////
    // Initializing the First Scene of the game (MainMenu) //
    /////////////////////////////////////////////////////////
    let mut scenes = Vec::with_capacity(10);
    scenes.push(MainMenu::new(render_tx.clone()));
    // Init First Scene
    let index = scenes.len() - 1;
    if let Some(scene) = scenes.get_mut(index) {
        if !scene.is_init() {
            scene.init(&mut ins, SceneInitSignals::None);
        }
    }

    ///////////////////////////////////
    // Starting the main thread loop //
    ///////////////////////////////////
    match main_loop(ins, scenes) {
        Ok(r) => Ok(r),
        Err(e) => Err(e),
    }
}

fn main_loop(mut ins: Instance, mut stack: Vec<Box<dyn Scene>>) -> Result<(), Error> {
    ///////////////////////////////////////////////
    // Adding Detla Frame variables to the stack //
    ///////////////////////////////////////////////
    let mut end_frame: Instant = Instant::now();
    let mut dt: f32 = Duration::from_millis(16).as_secs_f32(); // pretend 60fps until we can calculate real delta

    //////////////////////////////////////////////////////////////////////////
    // Main Game Loop (while context alive and there is a scene to process) //
    //////////////////////////////////////////////////////////////////////////
    while ins.ctx.is_alive() && stack.len() > 0 {
        //////////////////////////////
        // Update the current scene //
        //////////////////////////////
        let index = stack.len() - 1;
        let sig = stack.get_mut(index).unwrap().update(&mut ins, dt);

        ////////////////////////////////////////////
        // Handle any returned signals from scene //
        ////////////////////////////////////////////
        match dispatch(&mut ins, &mut stack, sig) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        //////////////////////////
        // Calculate Delta Time //
        //////////////////////////
        dt = (Instant::now() - end_frame).as_secs_f32();
        end_frame = Instant::now();
    }

    //////////////////////////////////////////////
    // Clean up after client starts to shutdown //
    //////////////////////////////////////////////
    exit_engine(ins.term_orig.clone())
}

fn dispatch(ins: &mut Instance, stack: &mut Vec<Box<dyn Scene>>, sig: EngineSignal) -> Result<(), Error> {
    match sig {
        EngineSignal::None => {}
        EngineSignal::Quit => ins.ctx.cancel(),
        EngineSignal::Scenes(ss) => match ss {
            SceneSignal::New { mut scene, signal } => {
                if stack.len() > 0 {
                    let index = stack.len() - 1;
                    stack.get_mut(index).unwrap().suspend(ins);
                }
                let sig = scene.init(ins, signal);
                match dispatch(ins, stack, sig) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
                stack.push(scene);
            }
            SceneSignal::Pop => {
                stack.pop();
                let index = stack.len() - 1;
                stack.get_mut(index).unwrap().resume(ins);
            }
        },
        EngineSignal::Render(msg) => match ins.render_queue.send(msg) {
            Ok(_) => {}
            Err(e) => match dispatch(ins, stack, EngineSignal::Log(e.to_string())) {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
        },
        EngineSignal::Batch(mut v) => {
            while v.len() > 0 {
                match dispatch(ins, stack, v.swap_remove(0)) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
        }
        EngineSignal::Sequence(mut v) => {
            while v.len() > 0 {
                match dispatch(ins, stack, v.remove(0)) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
        }
        EngineSignal::Error(e) => return Err(e),
        EngineSignal::Log(msg) => {}
    }
    Ok(())
}

fn start_audio_thread(ctx: Context, rx: mpsc::Receiver<AudioMsg>) -> JoinHandle<()> {
    spawn(move || audio_thread(ctx, rx))
}

fn start_event_thread(ctx: Context, tx: mpsc::Sender<Event>) -> JoinHandle<()> {
    spawn(move || event_thread(ctx, tx))
}

fn start_render_thread(
    ctx: Context,
    canvas: Canvas,
    event_tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<RenderSignal>,
    lg: Arc<logging::Logger>,
) -> JoinHandle<()> {
    spawn(move || render_thread(ctx, canvas, rx, event_tx, lg))
}

fn exit_engine(mut t: my_term::Terminal) -> Result<(), Error> {
    t.toggle_alt_buffer();
    t.toggle_cursor_visable();
    my_term::set_term(t);
    Ok(())
}

fn debug_pause() {
    print!("\x1b[0m\rPress any key to continue...\n\r");
    let mut buf: [u8; 1] = [0; 1];
    let _ = stdin().read(&mut buf);
}
