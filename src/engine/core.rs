use super::{
    AudioMsg, Context, Error, consts,
    input::{CLEAR_BUFFER, Event, InputBuffer, poll_event},
    render,
    term::{self, Terminal, set_term},
};

use std::{
    cell::RefCell,
    io::{Read, stdin},
    rc::{Rc, Weak},
    sync::mpsc::{self},
    thread::spawn,
    time::{Duration, Instant},
};

pub struct Instance<T: Scene<T>> {
    ctx: Context,
    term_orig: Terminal,
    game_state: Vec<T>,
    canvas: render::Canvas,
}

impl<T: Scene<T>> Instance<T> {
    pub fn new(init_scene: T, canvas: render::Canvas) -> Self {
        Self {
            ctx: Context::new(),
            term_orig: Terminal::default(),
            game_state: vec![init_scene],
            canvas: canvas,
        }
    }
    pub fn add_scene(&mut self, s: T) {
        self.game_state.push(s);
    }
}

impl<T: Scene<T>> Default for Instance<T> {
    fn default() -> Self {
        Self {
            ctx: Context::new(),
            term_orig: Terminal::default(),
            game_state: vec![],
            canvas: render::Canvas::new(30, 30),
        }
    }
}

impl<T: Scene<T>> Drop for Instance<T> {
    fn drop(&mut self) {
        set_term(self.term_orig);
        if self.term_orig.alt_buffer() {
            self.term_orig.toggle_alt_buffer();
        }
        if self.term_orig.cursor_visable() {
            self.term_orig.toggle_cursor_visable();
        }
    }
}

pub fn run<T: Scene<T>>(mut ins: Instance<T>) -> Result<(), Error> {
    ins.term_orig = term::set_raw();
    ins.term_orig.toggle_alt_buffer();
    ins.term_orig.toggle_cursor_visable();

    let audio_ctx = ins.ctx.child();
    let (audio_tx, audio_rx) = mpsc::channel::<AudioMsg>();
    let _audio = spawn(move || audio_thread(audio_ctx));

    let input_ctx = ins.ctx.child();
    let (input_tx, input_rx) = mpsc::channel::<Event>();
    let _input = spawn(move || input_handling(input_ctx, &input_tx));

    let (render_tx, render_rx) = mpsc::channel::<render::Msg>();
    let render_ctx = ins.ctx.child();
    let _render = spawn(move || {
        render(
            render_ctx,
            render::Canvas::new(ins.canvas.width, ins.canvas.height),
            render_rx,
        )
    });

    let mut end_frame: Instant = Instant::now();
    let mut dt: Duration = Duration::from_millis(16);
    let l = ins.game_state.len() - 1;
    if let Some(state) = ins.game_state.get_mut(l) {
        if !state.is_init() {
            state.init(&render_tx);
        }
    }
    while ins.ctx.is_alive() {
        if let Err(e) = update(&mut ins, dt.as_secs_f32(), &input_rx, &render_tx) {
            ins.ctx.cancel();
            continue;
            //return Err(e);
        }
        dt = Instant::now() - end_frame;
        end_frame = Instant::now();
    }

    let _ = _audio.join();
    let _ = _input.join();
    let _ = _render.join();

    #[cfg(debug_assertions)]
    {
        print!("Press any key to continue...\n\r");
        //let mut buf: KeyBuf = CLEAR_BUFFER;
        let mut buf: [u8; 1] = [0; 1];
        let _ = stdin().read(&mut buf);
    }
    Ok(())
}

fn input_handling(ctx: Context, ch: &mpsc::Sender<Event>) -> Result<(), Error> {
    while ctx.is_alive() {
        let mut buf: InputBuffer = CLEAR_BUFFER;
        match stdin().read(&mut buf) {
            Err(e) => return Err(Error::IO(e)), // Should log not return
            Ok(_) => {}
        }
        let mut seq: &[u8] = &buf[0..1];
        if buf[0] == b'\x1b' {
            let mut c = 0;
            for each in buf {
                c += 1;
                if each == b'\0' {
                    seq = &buf[0..c];
                    break;
                }
            }
        }
        let event = poll_event(&buf);
        match event {
            Some(e) => {
                if let Err(err) = ch.send(e) {
                    return Err(Error::SendEventError(err));
                }
            }
            None => {} // Log this
        }
    }
    #[cfg(debug_assertions)]
    {
        print!("Input Handling thread done\n\r");
    }
    Ok(())
}

fn update<T: Scene<T>>(
    ins: &mut Instance<T>,
    delta_time: f32,
    reciever: &mpsc::Receiver<Event>,
    render_send: &mpsc::Sender<render::Msg>,
) -> Result<(), Error> {
    let mut signals = None;
    let index = ins.game_state.len() - 1;
    if let Some(state) = ins.game_state.get_mut(index) {
        signals = Some(state.update(delta_time, reciever, render_send));
    }
    if let Some(signals) = signals {
        signal_dispatch(ins, signals, render_send);
    }

    Ok(())
}

fn signal_dispatch<T: Scene<T>>(
    ins: &mut Instance<T>,
    signal: Signal<T>,
    render_tx: &mpsc::Sender<render::Msg>,
) {
    match signal {
        Signal::None => {}
        Signal::Quit => ins.ctx.cancel(),
        Signal::PopScene => {
            ins.game_state.pop();
            let len = ins.game_state.len() - 1;
            ins.game_state.get_mut(len).unwrap().resume(render_tx);
        }
        Signal::NewScene(mut s) => {
            if ins.game_state.len() > 0 {
                let len = ins.game_state.len() - 1;
                ins.game_state.get_mut(len).unwrap().suspend(render_tx);
            }
            s.init(render_tx);
            ins.add_scene(s)
        }
        Signal::TerminalState(t) => ins.term_orig = t,
        Signal::Batch(mut v) => {
            while v.len() > 0 {
                signal_dispatch(ins, v.swap_remove(0), render_tx);
            }
        }
        // Same as Batch but garentees order
        Signal::Sequence(mut v) => {
            while v.len() > 0 {
                signal_dispatch(ins, v.remove(0), render_tx);
            }
        }
    }
}

fn render(
    ctx: Context,
    canvas: render::Canvas,
    reciever: mpsc::Receiver<render::Msg>,
) -> Result<(), Error> {
    let canvas_area = canvas.width * canvas.height;

    let mut buff_t: Vec<Option<Rc<RefCell<render::Object>>>> = Vec::with_capacity(canvas_area);
    let mut dyn_t: Vec<Weak<RefCell<render::Object>>> = Vec::with_capacity(canvas_area);
    let mut prefix: String = String::new();
    let mut suffix: String = String::new();
    let mut dirty: bool = true;
    buff_t.resize(canvas_area, None);

    while ctx.is_alive() {
        for msg in reciever.try_iter() {
            dirty = true;
            render::msg_dispatch(
                msg,
                &canvas,
                &mut prefix,
                &mut suffix,
                &mut buff_t,
                &mut dyn_t,
            );
        }

        // Clear invalid weak refs and updating dynamic objects
        let l = dyn_t.len();
        dyn_t.retain(|x| x.upgrade().is_some());
        if dyn_t.len() != l {
            dirty = true;
        }
        for obj in dyn_t.iter() {
            let obj = obj.upgrade().unwrap();
            let mut obj = obj.borrow_mut();
            if obj.is_dynamic() && obj.update() {
                dirty = true;
            }
        }

        // Print to Screen
        if dirty {
            let mut output = String::from("\x1b[0m");
            for (i, each) in buff_t.iter().enumerate() {
                let x: usize = (i % canvas.width) + 1;
                let y: usize = (i / canvas.width) + 1;
                let end: &str = if x == canvas.width {"\x1b[0m"} else {""};
                if let Some(each) = each {
                    output.push_str(&format!(
                        "\x1b[{};{}f{}{}{}{}",
                        y,
                        x,
                        prefix,
                        each.borrow().sprite(),
                        suffix,
                        end
                    ));
                } else {
                    output.push_str(&format!("\x1b[{};{}f ", y, x));
                }
            }
            //print!("\x1b[H\x1b[2J");
            //std::thread::sleep(Duration::from_millis(5));
            print!("{}", output);
            dirty = false;
        }
        std::thread::sleep(Duration::from_millis(5));
    }

    #[cfg(debug_assertions)]
    {
        print!("Render Thread Done\n\r");
    }
    Ok(())
}

fn audio_thread(ctx: Context) {
    while ctx.is_alive() {}
    #[cfg(debug_assertions)]
    {
        print!("Exiting Audio Thread\n\r");
    }
}

////////////////
///  TRAITS  ///
////////////////

pub trait Scene<T: Scene<T>> {
    fn update(
        &mut self,
        delta_time: f32,
        event: &mpsc::Receiver<Event>,
        render_tx: &mpsc::Sender<render::Msg>,
    ) -> Signal<T>;
    fn init(&mut self, render_tx: &mpsc::Sender<render::Msg>);
    fn is_init(&self) -> bool;
    fn suspend(&mut self, render_tx: &mpsc::Sender<render::Msg>);
    fn resume(&mut self, render_tx: &mpsc::Sender<render::Msg>);
    fn is_paused(&self) -> bool;
    fn reset(&mut self);
}

#[derive(Clone)]
pub enum Signal<T: Scene<T>> {
    None,
    Quit,
    PopScene,
    NewScene(T),
    TerminalState(Terminal),
    Batch(Vec<Signal<T>>),
    Sequence(Vec<Signal<T>>),
}

#[cfg(test)]
mod test {

    use crate::engine::types::Position;

    use super::{Context, render};

    use std::sync::mpsc;
    use std::thread::spawn;
    use std::time::Duration;

    #[test]
    fn render_pipline() {
        let mut root = Context::new();
        let (tx, rx) = mpsc::channel::<render::Msg>();
        let rctx = root.child();
        let h = spawn(move || render(rctx, render::Canvas::new(10, 10), rx));
        let _ = tx.send(render::Msg::Insert(
            Position::new(1, 1),
            render::Object::new_dynamic(
                vec![
                    String::from("\x1b[34mX"),
                    String::from("\x1b[35mд"),
                    String::from("\x1b[36mX"),
                ],
                Duration::from_millis(250),
            )
            .unwrap(),
        ));
        for _ in 0..10 {
            std::thread::sleep(Duration::from_millis(500));
            let _ = tx.send(render::Msg::Swap(Position::new(1, 1), Position::new(5, 5)));
            std::thread::sleep(Duration::from_millis(500));
            let _ = tx.send(render::Msg::Swap(Position::new(5, 5), Position::new(1, 1)));
        }
        root.cancel();
        let _ = h.join();
    }
}
