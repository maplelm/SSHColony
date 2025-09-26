#![deny(unused)]

#[cfg(not(test))]
use super::super::ui::style::{CLEAR_COLORS, CURSOR_HOME};
use super::super::{
    Context,
    input::Event,
    input::OtherEvent,
    render::{Canvas, Msg, Object},
    types::Position,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::mpsc,
};

type Grid = Vec<Option<Rc<RefCell<Object>>>>;
type DynRefList = Vec<Weak<RefCell<Object>>>;

pub fn render_thread(
    ctx: Context,
    mut canvas: Canvas,
    rx: mpsc::Receiver<Msg>,
    event_tx: mpsc::Sender<Event>,
) {
    let canvas_area = canvas.width * canvas.height;
    let mut object_grid: Grid = Vec::with_capacity(canvas_area);
    let mut dynamics_list: DynRefList = Vec::with_capacity(canvas_area);
    let mut tile_prefix: String = String::new();
    let mut tile_suffix: String = String::new();
    let mut dirty: bool = true;

    object_grid.resize(canvas_area, None);

    // Main Loop
    while ctx.is_alive() {
        dispatch_messages(
            &rx,
            &event_tx,
            &mut dirty,
            &mut canvas,
            &mut tile_prefix,
            &mut tile_suffix,
            &mut object_grid,
            &mut dynamics_list,
        );
        clear_invalid_weak_refs(&mut dynamics_list, &mut dirty);
        update_dynamic_objects(&mut dynamics_list, &mut dirty);
        print_grid_to_screen(
            &object_grid,
            &canvas,
            &tile_prefix,
            &tile_suffix,
            &mut dirty,
        );
    }
}

fn dispatch_messages(
    rx: &mpsc::Receiver<Msg>,
    event_tx: &mpsc::Sender<Event>,
    dirty: &mut bool,
    canvas: &mut Canvas,
    tile_prefix: &mut String,
    tile_suffix: &mut String,
    object_grid: &mut Grid,
    dynamics_list: &mut DynRefList,
) {
    for msg in rx.try_iter() {
        *dirty = true;
        process_each_msg(
            msg,
            event_tx,
            canvas,
            tile_prefix,
            tile_suffix,
            object_grid,
            dynamics_list,
        );
    }
}
fn process_each_msg(
    msg: Msg,
    event_tx: &mpsc::Sender<Event>,
    canvas: &mut Canvas,
    tile_prefix: &mut String,
    tile_suffix: &mut String,
    object_grid: &mut Grid,
    dynamics_list: &mut DynRefList,
) {
    match msg {
        Msg::Batch(batch) => batch_msg(
            batch,
            event_tx,
            canvas,
            tile_prefix,
            tile_suffix,
            object_grid,
            dynamics_list,
        ),
        Msg::TermSizeChange(c, r) => term_size_change_msg(c, r, canvas, object_grid, event_tx),
        Msg::Insert(pos, obj) => insert_msg(pos, obj, canvas, object_grid, dynamics_list),
        Msg::Prefix(prefix) => change_prefix_msg(prefix, tile_prefix),
        Msg::Suffix(suffix) => change_suffix_msg(suffix, tile_suffix),
        Msg::InsertRange { start, end, object } => {
            insert_range_msg(start, end, object, canvas, object_grid, dynamics_list)
        }
        Msg::InsertText {
            pos,
            text,
            ..
        } => insert_text_msg(
            pos,
            text,
            canvas,
            object_grid,
        ),
        Msg::Remove(pos) => remove_msg(pos, object_grid, canvas),
        Msg::RemoveRange(start, end) => remove_range_msg(start, end, object_grid, canvas),
        Msg::Swap(a, b) => swap_msg(a, b, canvas, object_grid),
        Msg::Clear => clear_msg(object_grid, dynamics_list),
    }
}

fn batch_msg(
    messages: Vec<Msg>,
    event_tx: &mpsc::Sender<Event>,
    canvas: &mut Canvas,
    tile_prefix: &mut String,
    tile_suffix: &mut String,
    object_grid: &mut Grid,
    dynamics_list: &mut DynRefList,
) {
    for msg in messages {
        process_each_msg(
            msg,
            event_tx,
            canvas,
            tile_prefix,
            tile_suffix,
            object_grid,
            dynamics_list,
        );
    }
}

fn term_size_change_msg(
    cols: u32,
    rows: u32,
    canvas: &mut Canvas,
    object_grid: &mut Grid,
    event_tx: &mpsc::Sender<Event>,
) {
    canvas.width = cols as usize;
    canvas.height = rows as usize;
    object_grid.resize((cols * rows) as usize, None);
    if let Err(_e) = event_tx.send(Event::Other(OtherEvent::ScreenSizeChange {
        width: canvas.width as u32,
        height: canvas.height as u32,
    })) {
        // log that render could not send the canvas change as an event
    }
}

fn insert_msg(
    pos: Position<usize>,
    object: Object,
    canvas: &Canvas,
    object_grid: &mut Grid,
    dynamics_list: &mut DynRefList,
) {
    match object {
        Object::Static { .. } => {
            object_grid[pos.y * canvas.width + pos.x] = Some(Rc::new(RefCell::new(object)))
        }
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

fn insert_range_msg(
    start: Position<usize>,
    end: Position<usize>,
    object: Object,
    canvas: &Canvas,
    object_grid: &mut Grid,
    dynamics_list: &mut DynRefList,
) {
    let process: fn(
        object: Object,
        offset: usize,
        x: usize,
        y: usize,
        canvas: &Canvas,
        grid: &mut Grid,
        dyn_list: &mut DynRefList,
    );
    match object {
        Object::Static { .. } => {
            process = |object: Object,
                       offset: usize,
                       x: usize,
                       y: usize,
                       canvas: &Canvas,
                       grid: &mut Grid,
                       _dyn_list: &mut DynRefList| {
                grid[offset + (y * canvas.width + x)] = Some(Rc::new(RefCell::new(object.clone())));
            };
        }
        Object::Dynamic { .. } => {
            process = |object: Object, offset, x, y, canvas, grid, dyn_list| {
                let val = Rc::new(RefCell::new(object.clone()));
                dyn_list.push(Rc::downgrade(&val));
                grid[offset + (y * canvas.width + x)] = Some(val);
            };
        }
    }
    for y in 0..(start.y as i32 - end.y as i32).abs() {
        for x in 0..(start.x as i32 - end.x as i32).abs() {
            process(
                object.clone(),
                start.y * canvas.width + start.x,
                x as usize,
                y as usize,
                canvas,
                object_grid,
                dynamics_list,
            );
        }
    }
}

fn insert_text_msg(
    pos: Position<usize>,
    text: String,
    canvas: &Canvas,
    object_grid: &mut Grid,
) {
    let mut y: usize = pos.y;
    let mut x: usize = pos.x;
    for each in text.chars() {
        if each == ' ' || each == '\n' {
            object_grid[y * canvas.width + x] = None;
        } else {
            object_grid[y * canvas.width + x] =
                Some(Rc::new(RefCell::new(Object::new_static(each, None, None))));
        }
        if each == '\n' {
            y += 1;
            x = pos.x
        } else {
            x += 1;
        }
    }
}

fn remove_msg(pos: Position<usize>, object_grid: &mut Grid, canvas: &Canvas) {
    object_grid[pos.y * canvas.width + pos.x] = None;
}

fn remove_range_msg(
    start: Position<usize>,
    end: Position<usize>,
    object_grid: &mut Grid,
    canvas: &Canvas,
) {
    for y in 0..(start.y as i32 - end.y as i32).abs() {
        for x in 0..(start.x as i32 - end.x as i32).abs() {
            object_grid
                [(start.y * canvas.width + start.x) + (y as usize * canvas.width + x as usize)] =
                None;
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

fn print_grid_to_screen(
    object_grid: &Grid,
    canvas: &Canvas,
    tile_prefix: &String,
    tile_suffix: &String,
    dirty: &mut bool,
) {
    if !*dirty {
        return;
    }
    #[cfg(not(test))]
    let mut output = String::from(CURSOR_HOME) + CLEAR_COLORS;
    #[cfg(test)]
    let mut output = String::new();
    for (i, c) in object_grid.iter().enumerate() {
        let x: usize = (i % canvas.width) + 1;
        let end: &str = if x == canvas.width { "\r\n" } else { "" };
        if let Some(object) = c {
            output.push_str(&format!(
                "{}{}{}{}",
                tile_prefix,
                object.borrow().sprite(),
                tile_suffix,
                end));
        } else {
            output.push_str(&format!(
                "{}{}{}{}",
                tile_prefix,
                " ",
                tile_suffix,
                end));
        }
    }
    print!("{}", output);
    *dirty = false;
}

#[cfg(test)]
mod test {
    use crate::engine::ui::{
        Border, BorderSprite, Menu, MenuItem, Padding,
        style::{Justify, Measure, Origin},
    };

    use super::*;
    #[test]
    fn test_insert_text_to_grid() {
        let c = Canvas::new(25, 10);
        let mut m = Menu::<(), ()>::new(
            0,
            0,
            Some(Measure::Cell(25)),
            Some(Measure::Cell(10)),
            Origin::TopLeft,
            Justify::Left,
            Some(Border::new(
                BorderSprite::String("#=:".to_string()),
                Padding::square(2),
            )),
            vec![
                MenuItem::new("Item One".to_string(), |_| None),
                MenuItem::new("Item Two".to_string(), |_| None),
                MenuItem::new("Item Three".to_string(), |_| None),
            ],
        );
        let m_output = m.output(&c);
        let mut grid: Grid = vec![];
        grid.resize(c.width * c.height, None);

        insert_text_msg(
            Position { x: m.x(), y: m.y() },
            m_output.clone(),
            &c,
            &mut grid,
        );
        println!("Raw Menu Output:\n\r{}\r", m_output);
        let fill = String::new();
        let mut b = true;
        println!("Grid Output:\r");
        print_grid_to_screen(&grid, &c, &fill, &fill, &mut b);
        let mut offset: usize = 0;
        for (i, c) in m_output.chars().enumerate() {
            if c == '\n' {
                offset += 1;
            }
            let grid_c: char = if let Some(c) = &grid[i] {
                c.borrow().sprite().symbol()
            } else {
                ' '
            };
            assert_eq!(grid_c, m_output.chars().nth(i - offset).unwrap())
        }
    }
}