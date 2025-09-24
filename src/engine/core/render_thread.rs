use super::super::{ui::style::CLEAR_COLORS, Context, input::Event, types::Position, input::OtherEvent, render::{Canvas, Msg, Object}};
use std::{cell::RefCell, process::Termination, rc::{Rc, Weak}, sync::mpsc};

type Grid= Vec<Option<Rc<RefCell<Object>>>>;
type DynRefList = Vec<Weak<RefCell<Object>>>;

pub fn render_thread(ctx: Context, mut canvas: Canvas, rx: mpsc::Receiver<Msg>, event_tx: mpsc::Sender<Event>) {
    let mut canvas_area = canvas.width * canvas.height;
    let mut object_grid: Grid = Vec::with_capacity(canvas_area);
    let mut dynamics_list: DynRefList = Vec::with_capacity(canvas_area);
    let mut tile_prefix: String = String::new();
    let mut tile_suffix: String = String::new();
    let mut dirty: bool = true;

    object_grid.resize(canvas_area, None);

    // Main Loop
    while ctx.is_alive() {
        dispatch_messages(&rx, &event_tx, &mut dirty, &mut canvas, &mut tile_prefix, &mut tile_suffix, &mut object_grid, &mut dynamics_list);
        clear_invalid_weak_refs(&mut dynamics_list, &mut dirty);
        update_dynamic_objects(&mut dynamics_list, &mut dirty);
        print_grid_to_screen(&object_grid, &canvas, &tile_prefix, &tile_suffix, &mut dirty);
    }
}

fn dispatch_messages(rx: &mpsc::Receiver<Msg>, event_tx: &mpsc::Sender<Event>, dirty: &mut bool, canvas: &mut Canvas, tile_prefix: &mut String, tile_suffix: &mut String, object_grid: &mut Grid, dynamics_list: &mut DynRefList) {
    for msg in rx.try_iter() {
        *dirty = true;
        process_each_msg(msg, event_tx, canvas, tile_prefix, tile_suffix, object_grid, dynamics_list);
    }
}
fn process_each_msg(msg: Msg, event_tx: &mpsc::Sender<Event>, canvas: &mut Canvas, tile_prefix: &mut String, tile_suffix: &mut String, object_grid: &mut Grid, dynamics_list: &mut DynRefList) {
    match msg {
        Msg::Batch(batch) => batch_msg(batch, event_tx, canvas, tile_prefix, tile_suffix, object_grid, dynamics_list),
        Msg::TermSizeChange(cols,rows ) => term_size_change_msg(cols, rows, canvas, event_tx),
        Msg::Insert(pos, obj) => insert_msg(pos, obj, canvas, object_grid, dynamics_list),
        Msg::Prefix(prefix) => change_prefix_msg(prefix, tile_prefix),
        Msg::Suffix(suffix) => change_suffix_msg(suffix, tile_suffix),
        Msg::InsertRange { start, end, object } => insert_range_msg(start, end, object, canvas, object_grid, dynamics_list),
        Msg::InsertText { pos, text, prefix, suffix } => insert_text_msg(pos, text, prefix.as_ref(), suffix.as_ref(), canvas, object_grid, dynamics_list),
        Msg::Remove(pos) => remove_msg(pos, object_grid, canvas),
        Msg::RemoveRange(start, end) => remove_range_msg(start, end, object_grid, canvas),
        Msg::Swap(a, b) => swap_msg(a, b, canvas, object_grid),
        Msg::Clear => clear_msg(object_grid, dynamics_list),
        _ => {todo!()}
    }
}

fn batch_msg(messages: Vec<Msg>, event_tx: &mpsc::Sender<Event>, canvas: &mut Canvas, tile_prefix: &mut String, tile_suffix: &mut String, object_grid: &mut Grid, dynamics_list: &mut DynRefList) {
    for msg in messages {
        process_each_msg(msg, event_tx, canvas, tile_prefix, tile_suffix, object_grid, dynamics_list);
    }
}

fn term_size_change_msg(cols: u32, rows: u32, canvas: &mut Canvas, event_tx: &mpsc::Sender<Event>) {
    canvas.width = cols as usize;
    canvas.height = rows as usize;
    if let Err(e) = event_tx.send(Event::Other(OtherEvent::ScreenSizeChange { width: canvas.width as u32, height: canvas.height as u32})){
        // log that render could not send the canvas change as an event
    }
}

fn insert_msg(pos: Position<usize>, object: Object, canvas: &Canvas, object_grid: &mut Grid, dynamics_list: &mut DynRefList) {
    match object {
        Object::Static{ .. } => object_grid[pos.y * canvas.width + pos.x] = Some(Rc::new(RefCell::new(object))),
        Object::Dynamic { .. } => {
            let val = Rc::new(RefCell::new(object));
            dynamics_list.push(Rc::downgrade(&val));
            object_grid[pos.y * canvas.width + pos.x] = Some(val);
        }
    }
}

fn change_prefix_msg(new: String, tile_prefix: &mut String) {
    *tile_prefix = new;
}

fn change_suffix_msg(new: String, tile_suffix: &mut String) {
    *tile_suffix = new;
}

fn insert_range_msg( start: Position<usize>, end: Position<usize>, object: Object, canvas: &Canvas, object_grid: &mut Grid, dynamics_list: &mut DynRefList ) {
    let mut process: fn(object: Object, offset: usize, x: usize, y: usize, canvas: &Canvas, grid: &mut Grid, dyn_list: &mut DynRefList); 
    match object {
        Object::Static{ .. } => {
            process = |object: Object, offset: usize, x: usize, y: usize, canvas: &Canvas, grid: &mut Grid, dyn_list: &mut DynRefList| {
                grid[offset + (y * canvas.width + x)] = Some(Rc::new(RefCell::new(object.clone())));
            };
        },
        Object::Dynamic{ .. } => {
            process = |object: Object, offset, x, y, canvas, grid, dyn_list| {
                let val = Rc::new(RefCell::new(object.clone()));
                dyn_list.push(Rc::downgrade(&val));
                grid[offset + (y * canvas.width + x)] = Some(val);
            };
        },
    }
    for y in 0..(start.y as i32 - end.y as i32).abs() {
        for x in 0..(start.x as i32 - end.x as i32).abs() {
            process(object.clone(), start.y * canvas.width + start.x, x as usize, y as usize, canvas, object_grid, dynamics_list);
        }
    }
}

fn insert_text_msg (pos: Position<usize>, text: String, prefix: Option<&String>, suffix: Option<&String>, canvas: &Canvas, object_grid: &mut Grid, dynamics_list: &mut DynRefList) {
    let mut y: usize = pos.y;
    let mut x: usize = pos.x;
    for each in text.chars() {
        if !each.is_ascii_graphic() || each == ' ' || each == '\n' {
            object_grid[y * canvas.width + x] = None;
        } else {
            object_grid[y * canvas.width + x] = Some(Rc::new(RefCell::new(sprite_from_char(each, prefix, suffix))));
        }
        if each == '\n' {
            y += 1;
            x = pos.x
        } else {
            x += 1;
        }
    }

}

fn sprite_from_char(c: char, prefix: Option<&String>, suffix: Option<&String>) -> Object {
    let mut new = String::from(if let Some(p) = prefix{p.as_str()} else {""});
    new.push(c);
    new.push_str(if let Some(s) = suffix {s}else{""});
    Object::new_static(new).unwrap()
}

fn remove_msg(pos: Position<usize>, object_grid: &mut Grid, canvas: &Canvas) {
    object_grid[pos.y * canvas.width + pos.x] = None;
}

fn remove_range_msg(start: Position<usize>, end: Position<usize>, object_grid: &mut Grid, canvas: &Canvas) {
    for y in 0..(start.y as i32 - end.y as i32).abs() {
        for x in 0..(start.x as i32 - end.x as i32).abs() {
            object_grid[(start.y * canvas.width + start.x) + (y as usize * canvas.width + x as usize)] = None;
        }
    }
}

fn swap_msg(a: Position<usize>, b: Position<usize>, canvas: &Canvas, object_grid: &mut Grid) {
    object_grid.swap(a.y * canvas.width + a.x, b.y * canvas.width + b.x);
}

fn clear_msg(object_grid: &mut Grid, dynamics_list: &mut DynRefList) {
    object_grid.fill(None);
    dynamics_list.clear();
}

fn clear_invalid_weak_refs(dynamics_list: &mut DynRefList, dirty: &mut bool) {
    let original_len = dynamics_list.len();
    dynamics_list.retain(|x| x.upgrade().is_some());
    if original_len != dynamics_list.len() {
        *dirty = true;
    }
}

fn update_dynamic_objects(dynamics_list: &mut DynRefList, dirty: &mut bool) {
    for each in dynamics_list.iter() {
        let each = each.upgrade().unwrap();
        let mut each = each.borrow_mut();
        if each.update() {
            *dirty = true;
        }
    }
}

fn print_grid_to_screen(object_grid: &Grid, canvas: &Canvas, tile_prefix: &String, tile_suffix: &String, dirty: &mut bool){
    if !*dirty{
        return;
    }
    let mut output = String::from(CLEAR_COLORS);
    for (i, c) in object_grid.iter().enumerate() {
        let x: usize = (i % canvas.width) + 1;
        let y: usize = (i / canvas.width) + 1;
        let end: &str = if x == canvas.width {CLEAR_COLORS} else {""};
        let sprite = if c.is_some() {String::from(c.as_ref().unwrap().borrow().sprite())} else {String::from(" ")};
        output.push_str(&format!("\x1b[{};{}f{}{}{}{}", y, x, tile_prefix, sprite, tile_suffix, end));
    }
    print!("{}", output);
    *dirty = false;
}