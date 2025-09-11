use super::{
    AudioMsg,
    Context,
    Error,
    RenderMsg,
    consts,
    input::{ Event, InputBuffer, CLEAR_BUFFER, poll_event },
    render::{ self, Canvas, Object, render_msg_disbatch },
    term::{ self, Terminal, set_term }
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
    canvas: Canvas
}

impl<T: Scene<T>> Instance<T> {
    pub fn new(init_scene: T, canvas: Canvas) -> Self {
        Self{
            ctx: Context::new(),
            term_orig: Terminal::default(),
            game_state: vec![init_scene],
            canvas: canvas
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
            canvas: Canvas { width: 30, height: 30 }
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

    let (render_tx, render_rx) = mpsc::channel::<RenderMsg>();
    let render_ctx = ins.ctx.child();
    let _render = spawn(move || {
        render(
            render_ctx,
            Canvas{ width: ins.canvas.width, height: ins.canvas.height},
            render_rx,
        )
    });

    let mut end_frame: Instant = Instant::now();
    let mut dt: Duration = Duration::from_millis(16);
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
        print!("\x1b[2;30f\x1b[0K{:?}", seq);
        let event = poll_event(&buf);
        match event {
            Some(e) => {
                print!("\x1b[1;30f\x1b[0KEvent: {:?}", e);
                print!("Sending Event: {:?}\n\r", e);
                if let Err(err) = ch.send(e) {
                    return Err(Error::SendEventError(err));
                }
            }
            None => {
                print!("\x1b[1;30f\x1b[0Kpoll_event returned None!\n\r");
            } // Log this
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
    render_send: &mpsc::Sender<RenderMsg>,
) -> Result<(), Error> {
    let index = ins.game_state.len()-1;
    if let Some(state) = ins.game_state.get_mut(index) {
        match state.update(delta_time, reciever, render_send){
            Signal::None => {}
            Signal::Quit => ins.ctx.cancel(),
            Signal::NewScene(s) => ins.game_state.push(s),
            Signal::TerminalState(t) => ins.term_orig = t
        }
    }
    
    Ok(())
}

fn render(ctx: Context, canvas: Canvas, reciever: mpsc::Receiver<RenderMsg>) -> Result<(), Error> {
    let canvas_area = canvas.width * canvas.height;
    let mut buff_t: Vec<Option<Rc<RefCell<Object>>>> = Vec::with_capacity(canvas_area);
    let mut dyn_t: Vec<Weak<RefCell<Object>>> = Vec::with_capacity(canvas_area);
    buff_t.resize(canvas_area, None);

    while ctx.is_alive() {
        for msg in reciever.try_iter() {
            render_msg_disbatch(msg, &canvas, &mut buff_t, &mut dyn_t);
        }

        // Clear invalid weak refs and updating dynamic objects
        dyn_t.retain(|x| x.upgrade().is_some());
        for obj in dyn_t.iter() {
            let obj = obj.upgrade().unwrap();
            let mut obj = obj.borrow_mut();
            let obj = obj.as_dynamic().unwrap();
            obj.update();
        }

        // Print to Screen
        let mut output = String::new();
        for (i, each) in buff_t.iter().enumerate() {
            let x: usize = i % canvas.width;
            let y: usize = i / canvas.width;
            if let Some(each) = each {
                output.push_str(&format!("\x1b[{};{}f{}", y, x, each.borrow().sprite()));
            } else {
                output.push_str(&format!("\x1b[{};{}f\x1b[0m ", y, x));
            }
        }
        print!("{}", output);
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
        render_tx: &mpsc::Sender<RenderMsg>,
    ) -> Signal<T>;
    fn init(&mut self);
    fn suspend(&mut self);
    fn resume(&mut self);
    fn is_paused(&self) -> bool;
    fn reset(&mut self);
}

pub enum Signal<T: Scene<T>> {
    None,
    Quit,
    NewScene(T),
    TerminalState(Terminal)
}


#[cfg(test)]
mod test {

    use crate::engine::core::render;
    use crate::engine::render::{Canvas, DynamicObject, ObjectMove, ObjectPos, RenderMsg};

    use super::Context;
    use std::sync::mpsc;
    use std::thread::spawn;
    use std::time::Duration;

    #[test]
    fn render_pipline() {
        let mut root = Context::new();
        let (tx, rx) = mpsc::channel::<RenderMsg>();
        let rctx = root.child();
        let h = spawn(move || {
            render(
                rctx,
                Canvas {
                    width: 10,
                    height: 10,
                },
                rx,
            )
        });
        let _ = tx.send(RenderMsg::Insert(
            ObjectPos { x: 1, y: 1 },
            render::Object::Dynamic(
                DynamicObject::new(
                    vec![
                        String::from("\x1b[34mX"),
                        String::from("\x1b[35mд"),
                        String::from("\x1b[36mX"),
                    ],
                    Duration::from_millis(250),
                )
                .unwrap(),
            ),
        ));
        for _ in 0..10 {
            std::thread::sleep(Duration::from_millis(500));
            let _ = tx.send(RenderMsg::Move(ObjectMove {
                old: ObjectPos { x: 1, y: 1 },
                new: ObjectPos { x: 5, y: 5 },
            }));
            std::thread::sleep(Duration::from_millis(500));
            let _ = tx.send(RenderMsg::Move(ObjectMove {
                old: ObjectPos { x: 5, y: 5 },
                new: ObjectPos { x: 1, y: 1 },
            }));
        }
        root.cancel();
        let _ = h.join();
    }
}
