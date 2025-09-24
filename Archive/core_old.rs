use crate::engine::render::Canvas;
use super::{
    AudioMsg, Context, Error, consts,
    input::{CLEAR_BUFFER, Event, InputBuffer, poll_event},
    render,
    term::{self, Terminal, set_term, term_size},
};
use std::{
    cell::RefCell,
    io::{Read, stdin},
    rc::{Rc, Weak},
    sync::mpsc::{self},
    thread::spawn,
    time::{Duration, Instant},
};


fn render<T: Scene<T>>(
    ctx: Context,
    mut canvas: render::Canvas,
    reciever: mpsc::Receiver<render::Msg>,
) -> Result<(), Error> {
    let mut canvas_area = canvas.width * canvas.height;

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
                engine_tx,
                &mut canvas,
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
                let end: &str = if x == canvas.width { "\x1b[0m" } else { "" };
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
