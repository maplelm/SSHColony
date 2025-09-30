use crate::engine::{core::consts::DEFAULT_CANVAS, enums::SceneSignal};

use super::super::{
    AudioMsg, Context, Error,
    input::Event,
    render::{Canvas, Msg},
};
use super::{
    audio_thread::audio_thread, enums::Signal, event_thread::event_thread, instance::Instance,
    render_thread::render_thread, traits::Scene,
};
use std::thread::{JoinHandle, spawn};
use std::{
    io::{Read, stdin},
    sync::mpsc,
    time::{Duration, Instant},
};

pub fn start<T: Scene<T>>(mut ins: Instance<T>) -> Result<(), Error> {
    ins.term_orig = term::set_raw();
    ins.term_orig.toggle_alt_buffer();
    ins.term_orig.toggle_cursor_visable();

    let (audio_tx, audio_rx, audio_handle) = start_audio_thread(&ins.ctx);
    let (event_tx, event_rx, event_handle) = start_event_thread(&ins.ctx);
    let (render_tx, render_handle) = start_render_thread(&ins.ctx, ins.canvas.clone(), event_tx);
    // Init First Scene
    let index = ins.game_state.len() - 1;
    if let Some(scene) = ins.game_state.get_mut(index) {
        if !scene.is_init() {
            scene.init(&render_tx, &ins.canvas);
        }
    }

    #[cfg(not(debug_assertions))]
    {
        match main_loop(ins, render_tx, event_rx) {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
    #[cfg(debug_assertions)]
    {
        let res = main_loop(ins, render_tx, event_rx);
        debug_pause();
        match res {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}

fn main_loop<T: Scene<T>>(
    mut ins: Instance<T>,
    render_tx: mpsc::Sender<Msg>,
    event_rx: mpsc::Receiver<Event>,
) -> Result<(), Error> {
    let mut end_frame: Instant = Instant::now();
    let mut dt: f32 = Duration::from_millis(16).as_secs_f32(); // pretend 60fps until we can calculate real delta
    while ins.ctx.is_alive() && ins.game_state.len() > 0 {
        let index = ins.game_state.len() - 1;
        let signals =
            ins.game_state
                .get_mut(index)
                .unwrap()
                .update(dt, &event_rx, &render_tx, &ins.canvas);
        match dispatch(ins, signals, &render_tx) {
            Ok(i) => ins = i,
            Err(e) => return Err(e),
        }
        dt = (Instant::now() - end_frame).as_secs_f32();
        end_frame = Instant::now();
    }
    exit_engine(ins.term_orig.clone());
    Ok(())
}

fn dispatch<T: Scene<T>>(
    mut ins: Instance<T>,
    sig: Signal<T>,
    render_tx: &mpsc::Sender<Msg>,
) -> Result<Instance<T>, Error> {
    match sig {
        Signal::None => {}
        Signal::Quit => ins.ctx.cancel(),
        Signal::Scenes(ss) => {
            match ss {
                SceneSignal::New(mut ns) => {
                    if ins.game_state.len() > 0 {
                        let index = ins.game_state.len() - 1;
                        ins.game_state.get_mut(index).unwrap().suspend(render_tx);
                    }
                    let sig = ns.init(render_tx, &ins.canvas);
                    match dispatch(ins, sig, render_tx) {
                        Ok(i) => ins = i,
                        Err(e) => return Err(e),
                    }
                    ins.add_scene(ns);

                        }
                SceneSignal::Pop => {
                    ins.game_state.pop();
                    let index = ins.game_state.len() - 1;
                    ins.game_state
                        .get_mut(index)
                        .unwrap()
                        .resume(render_tx, &ins.canvas);
                }
            }
        }
        Signal::Batch(mut v) => {
            while v.len() > 0 {
                match dispatch(ins, v.swap_remove(0), render_tx) {
                    Ok(i) => ins = i,
                    Err(e) => return Err(e),
                }
            }
        }
        Signal::Sequence(mut v) => {
            while v.len() > 0 {
                match dispatch(ins, v.remove(0), render_tx) {
                    Ok(i) => ins = i,
                    Err(e) => return Err(e),
                }
            }
        }
        Signal::Error(e) => return Err(e),
        Signal::Log(msg) => {}
    }
    Ok(ins)
}

fn start_audio_thread(
    root_ctx: &Context,
) -> (
    mpsc::Sender<AudioMsg>,
    mpsc::Receiver<AudioMsg>,
    JoinHandle<()>,
) {
    let (tx, rx) = mpsc::channel();
    let child = root_ctx.child();
    let handle = spawn(move || audio_thread(child));
    (tx, rx, handle)
}

fn start_event_thread(
    root_ctx: &Context,
) -> (mpsc::Sender<Event>, mpsc::Receiver<Event>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();
    let child = root_ctx.child();
    let handle = spawn(move || event_thread(child, tx_clone));
    (tx, rx, handle)
}

fn start_render_thread(
    root_ctx: &Context,
    canvas: Canvas,
    event_tx: mpsc::Sender<Event>,
) -> (mpsc::Sender<Msg>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();
    let child = root_ctx.child();
    let canvas_move = canvas.clone();
    let event_tx_clone = event_tx.clone();
    let handle = spawn(move || render_thread(child, canvas, rx, event_tx_clone));
    (tx, handle)
}

fn exit_engine(mut t: term::Terminal) {
    t.toggle_alt_buffer();
    t.toggle_cursor_visable();
    term::set_term(t);
}

fn debug_pause() {
    print!("\x1b[0m\rPress any key to continue...\n\r");
    let mut buf: [u8; 1] = [0; 1];
    let _ = stdin().read(&mut buf);
}
