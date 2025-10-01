#![deny(unused)]

#[cfg(not(test))]
use super::super::ui::style::{CLEAR_COLORS, CURSOR_HOME};
use super::super::{
    Context,
    input::Event,
    enums::RenderSignal,
    input::OtherEvent,
    render::{Canvas, Layer, Object, RenderUnit, RenderUnitId},
    types::{SparseSet, Position3D},
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::{Arc, mpsc},
};
use term::color::{Background, Color, Foreground};

 //////////////////
 // Type Aliases //
 //////////////////
type Grid = SparseSet<RenderUnit>;
type DynRefList = Vec<Weak<RefCell<Object>>>;

////////////////////////////
// Main Loop For Renderer //
////////////////////////////

pub fn render_thread(
    ctx: Context,
    mut canvas: Canvas,
    rx: mpsc::Receiver<RenderSignal>,
    event_tx: mpsc::Sender<Event>,
) {

    let mut bg_counter: usize = 0;
    let mut mg_counter: usize = 0;
    let mut fg_counter: usize = 0;
    let mut free_bg: Vec<usize> = Vec::new();
    let mut free_mg: Vec<usize> = Vec::new();
    let mut free_fg: Vec<usize> = Vec::new();
    let mut background: Grid = SparseSet::new(1000);
    let mut middleground: Grid = SparseSet::new(1000);
    let mut foreground: Grid = SparseSet::new(1000);
    let mut dynamics_list: DynRefList = Vec::new();
    let mut foreground_color: Foreground = Foreground::new(Color::None);
    let mut background_color: Background = Background::new(Color::None);
    let mut dirty: bool = true;


    // Main Loop
    while ctx.is_alive() {
        check_for_signals(
            &mut foreground,
            &mut middleground,
            &mut background,
            &mut dynamics_list,
            &mut fg_counter,
            &mut mg_counter,
            &mut bg_counter,
            &mut free_fg,
            &mut free_mg,
            &mut free_bg,
            &rx,
            &event_tx,
            &mut dirty,
            &mut canvas,
            &mut foreground_color,
            &mut background_color,
        );
        clear_invalid_weak_refs(&mut dynamics_list, &mut dirty);
        update_dynamic_objects(&mut dynamics_list, &mut dirty);
        print(&background, &middleground, &foreground, &canvas, &foreground_color, &background_color, &mut dirty);
    }
}

fn check_for_signals(
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    dyn_list: &mut DynRefList,
    fg_counter: &mut usize,
    mg_counter: &mut usize,
    bg_counter: &mut usize,
    free_fg: &mut Vec<usize>,
    free_mg: &mut Vec<usize>,
    free_bg: &mut Vec<usize>,
    rx: &mpsc::Receiver<RenderSignal>,
    event_tx: &mpsc::Sender<Event>,
    dirty: &mut bool,
    canvas: &mut Canvas,
    fg_color: &mut Foreground,
    bg_color: &mut Background,
) {
    for msg in rx.try_iter() {
        *dirty = true;
        dispatch_msg(
            msg, 
            fg, mg, bg,
            dyn_list,
            fg_counter, mg_counter, bg_counter,
            free_fg, free_mg, free_bg,
            event_tx,
            canvas,
            fg_color, bg_color,
        );
    }
}
fn dispatch_msg(
    msg: RenderSignal,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    dyn_list: &mut DynRefList,
    fg_counter: &mut usize,
    mg_counter: &mut usize,
    bg_counter: &mut usize,
    free_fg: &mut Vec<usize>,
    free_mg: &mut Vec<usize>,
    free_bg: &mut Vec<usize>,
    event_tx: &mpsc::Sender<Event>,
    canvas: &mut Canvas,
    fg_color: &mut Foreground,
    bg_color: &mut Background,
) {
    match msg {
        RenderSignal::Batch(mut batch) => batch_msg(&mut batch, fg, mg, bg, dyn_list, fg_counter, mg_counter, bg_counter,free_fg, free_mg, free_bg, event_tx, canvas, fg_color, bg_color),
        RenderSignal::Sequence(mut seq) => Sequence_msg(&mut seq, fg, mg, bg, dyn_list, fg_counter, mg_counter, bg_counter, free_fg, free_mg, free_bg, event_tx, canvas, fg_color, bg_color),
        RenderSignal::TermSizeChange(c, r) => term_size_change_msg(c, r, canvas, event_tx),
        RenderSignal::Insert(id_holder, new_obj) => insert(id_holder, new_obj, fg_counter, mg_counter, bg_counter, free_fg, free_mg, free_bg, fg, mg, bg, dyn_list),
        RenderSignal::Background(bg) => change_bg(bg, bg_color),
        RenderSignal::Foreground(fg) => change_fg(fg, fg_color),
        RenderSignal::Remove(key) => match key { RenderUnitId::Background(_) => remove(key, bg), RenderUnitId::Middleground(_) => remove(key, mg), RenderUnitId::Foreground(_) => remove(key, fg)},
        RenderSignal::Clear => clear_msg(fg, mg, bg, dyn_list),
        RenderSignal::Redraw => {}, // Used to mark display as dirty
        RenderSignal::Move(id, pos) => move_object(id, pos, fg, mg, bg),
        RenderSignal::MoveLayer(id, layer) => move_layer(id, layer, fg, mg, bg),
    }
}

// ! TODO: Need to add camera object to the renderer so that I can make sure that all the things are where they need to be
fn print(
    bg: &Grid,
    mg: &Grid,
    fg: &Grid,
    canvas: &Canvas,
    fg_color: &Foreground,
    bg_color: &Background,
    dirty: &mut bool,
) {
    if !*dirty {
        return;
    }

    let default_color: String = fg_color.to_ansi() + &bg_color.to_ansi();

    #[cfg(not(test))]
    let mut output = String::from(CURSOR_HOME) + CLEAR_COLORS;
    #[cfg(test)]
    let mut output = String::new();
    for (i, c) in object_grid.iter().enumerate() {
        let x: usize = (i % canvas.width) + 1;
        let end: &str = if x == canvas.width { "\r\n" } else { "" };
        if let Some(object) = c {
            output.push_str(default_color.as_str());
            output.push_str(&object.borrow().sprite().to_string());
            output.push_str(end);
        } else {
            output.push_str(default_color.as_str());
            output.push(' ');
            output.push_str(end);
        }
    }
    print!("{}", output);
    *dirty = false;
}

//////////////////////
// Helper Functions //
//////////////////////
fn batch_msg(
    messages: &mut Vec<RenderSignal>,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    dyn_list: &mut DynRefList,
    fg_counter: &mut usize,
    mg_counter: &mut usize,
    bg_counter: &mut usize,
    free_fg: &mut Vec<usize>,
    free_mg: &mut Vec<usize>,
    free_bg: &mut Vec<usize>,
    event_tx: &mpsc::Sender<Event>,
    canvas: &mut Canvas,
    fg_color: &mut Foreground,
    bg_color: &mut Background,
) {
    while messages.len() > 0 {
        dispatch_msg(
            messages.swap_remove(0),
            fg,
            mg,
            bg,
            dyn_list,
            fg_counter,
            mg_counter,
            bg_counter,
            free_fg,
            free_mg,
            free_bg,
            event_tx,
            canvas,
            fg_color,
            bg_color,
        );
    }
}

fn Sequence_msg(
    messages: &mut Vec<RenderSignal>,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    dyn_list: &mut DynRefList,
    fg_counter: &mut usize,
    mg_counter: &mut usize,
    bg_counter: &mut usize,
    free_fg: &mut Vec<usize>,
    free_mg: &mut Vec<usize>,
    free_bg: &mut Vec<usize>,
    event_tx: &mpsc::Sender<Event>,
    canvas: &mut Canvas,
    fg_color: &mut Foreground,
    bg_color: &mut Background,
) {
    while messages.len() > 0 {
        dispatch_msg(
            messages.remove(0),
            fg,
            mg,
            bg,
            dyn_list,
            fg_counter,
            mg_counter,
            bg_counter,
            free_fg,
            free_mg,
            free_bg,
            event_tx,
            canvas,
            fg_color,
            bg_color,
        );
    }
}

fn change_bg(new: Background, bg: &mut Background) {
    *bg = new;
}

fn change_fg(new: Foreground, fg: &mut Foreground) {
    *fg = new;
}

fn term_size_change_msg(
    cols: u32,
    rows: u32,
    canvas: &mut Canvas,
    event_tx: &mpsc::Sender<Event>,
) {
    canvas.width = cols as usize;
    canvas.height = rows as usize;
    if let Err(_e) = event_tx.send(Event::Other(OtherEvent::ScreenSizeChange {
        width: canvas.width as u32,
        height: canvas.height as u32,
    })) {
        // log that render could not send the canvas change as an event
    }
}

fn insert(
    id_holder: Arc<RenderUnitId>,
    new_object: Object,
    fg_counter: &mut usize,
    mg_counter: &mut usize,
    bg_counter: &mut usize,
    free_fg: &mut Vec<usize>,
    free_mg: &mut Vec<usize>,
    free_bg: &mut Vec<usize>,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    dyn_list: &mut DynRefList
) {

    let static_process = |set: &mut SparseSet<RenderUnit>, id_counter: &mut usize, free_ids: &mut Vec<usize>, id_holder: Arc<RenderUnitId>, object: Object|{
        let new_unit: RenderUnit = RenderUnit { id:id_holder, object: Rc::new(RefCell::new(object)) };
        new_unit.id.store(if free_ids.len() > 0 {free_ids.swap_remove(0)} else {*id_counter += 1; *id_counter -1});
        set.insert(new_unit.id.load().0, new_unit);
    };
    let dynamic_process = |dyn_list: &mut DynRefList, set: &mut SparseSet<RenderUnit>, id_counter: &mut usize, free_ids: &mut Vec<usize>, id_holder: Arc<RenderUnitId>, object: Object| {
        let new_unit: RenderUnit = RenderUnit { id: id_holder, object: Rc::new(RefCell::new(object)) };
        new_unit.id.store(if free_ids.len() > 0 {free_ids.swap_remove(0)} else {*id_counter += 1; *id_counter -1});
    dyn_list.push(Rc::downgrade(&new_unit.object));
    set.insert(new_unit.id.load().0, new_unit);
    };
        
    match new_object {
        Object::Static(_) =>{
            match new_object.layer() {
                Layer::Background => static_process(bg, bg_counter, free_bg, id_holder, new_object),
                Layer::Middleground => static_process(mg, mg_counter, free_mg, id_holder, new_object),
                Layer::Foreground => static_process(fg, fg_counter, free_fg, id_holder, new_object),
            }
        }
        Object::Dynamic(_) => {
            match new_object.layer() {
                Layer::Background => dynamic_process(dyn_list, bg, bg_counter, free_bg, id_holder, new_object),
                Layer::Middleground => dynamic_process(dyn_list, mg, mg_counter, free_mg, id_holder, new_object),
                Layer::Foreground => dynamic_process(dyn_list, fg, fg_counter, free_fg, id_holder, new_object),
            }
        }
    }
}



fn remove(key: RenderUnitId, list: &mut Grid) {
    list.remove(key.load().0);
}

fn clear_msg(fg: &mut Grid, mg : &mut Grid, bg: &mut Grid, dynamics_list: &mut DynRefList) {
    fg.clear();
    mg.clear();
    bg.clear();
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
        let mut each = each.upgrade().unwrap();
        each.
        if each.update() {
            *dirty = true;
        }
    }
}

fn move_object(id: RenderUnitId, pos: Position3D<i32>, fg: &mut Grid, mg: &mut Grid, bg: &mut Grid) {
    let (id, layer) = id.load();
    match layer {
        Layer::Background => bg.get(id).unwrap(),
        Layer::Middleground => mg.get(id).unwrap(),
        Layer::Foreground => fg.get(id).unwrap(),
    }.object.borrow_mut().shift(pos);
}

fn move_layer(id: RenderUnitId, layer: Layer, fg: &mut Grid, mg: &mut Grid, bg: &mut Grid) {
    let (id, current_layer) = id.load();
    let mut id = id;
    let mut changed = false;
    let val = match current_layer {
        Layer::Background => bg.remove(id).unwrap(),
        Layer::Middleground => mg.remove(id).unwrap(),
        Layer::Foreground => fg.remove(id).unwrap()
    };

    match layer {
        Layer::Background => {
            while bg.is_filled(id) {
                id += 1;
                if !changed {
                    changed = true;
                }
            }
            if changed {
                val.id.store(id);
            }
            bg.insert(id, val);
        }
        Layer::Middleground => {
            while mg.is_filled(id) {
                id += 1;
                if !changed {
                    changed = true;
                }
            }
            if changed {
                val.id.store(id);
            }
            mg.insert(id, val);
        }
        Layer::Foreground => {
            while fg.is_filled(id) {
                id += 1;
                if !changed {
                    changed = true;
                }
            }
            if changed {
                val.id.store(id);
            }
            fg.insert(id, val)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::engine::ui::style::Measure;
    use crate::engine::ui::{
        Border, BorderSprite, Menu, MenuItem, Padding,
        style::{Justify, Origin},
    };

    #[test]
    fn test_insert_text_to_grid() {
        let c = Canvas::new(25, 10);

        // Setting up Menu Object
        let mut m = Menu::<(), ()>::new(
            0,
            0,
            Some(Measure::Cell(25)),
            Some(Measure::Cell(10)),
            Origin::TopLeft,
            Justify::Left,
            Some(Border::from(
                BorderSprite::String("#=:".to_string()),
                Padding::square(2),
            )),
            vec![
                MenuItem::new("Item One".to_string(), |_| None),
                MenuItem::new("Item Two".to_string(), |_| None),
                MenuItem::new("Item Three".to_string(), |_| None),
            ],
        );
        // Setting up Grid Object
        let mut grid: Grid = vec![];
        grid.resize(c.width * c.height, None);

        insert_text_msg(
            Position { x: m.x(), y: m.y() },
            match m.output(&c) {
                Some(out) => out,
                None => "".to_string(),
            },
            &c,
            &mut grid,
        );

        println!(
            "Raw Menu Output:\n{}",
            match m.output(&c) {
                Some(out) => out,
                None => "".to_string(),
            }
        );
        let fg = Foreground::new(Color::None);
        let bg = Background::new(Color::None);
        let mut b = true;
        println!("Grid Output:\r");
        print(&grid, &c, &fg, &bg, &mut b);
        for (i, ch) in match m.output(&c) {
            Some(out) => out,
            None => "".to_string(),
        }
        .replace("\n", "")
        .chars()
        .enumerate()
        {
            let x: usize = (i % c.width) + 1;
            let y: usize = (i / c.width) + 1;
            let grid_c: char = if let Some(c) = &grid[i] {
                c.borrow().sprite().symbol()
            } else {
                ' '
            };
            assert_eq!(grid_c, ch, "{x},{y}")
        }
    }
}
