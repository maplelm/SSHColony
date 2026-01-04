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

#![deny(unused)]

use crate::engine::render::{Camera, GlyphType, ObjectData, TextType};
#[allow(unused)]
use std::fmt::Write;
use std::io::{Stdout, Write as iowrite};

use super::super::ui::style::CLEAR_COLORS;
use super::super::{
    Context,
    enums::RenderSignal,
    input::{Event, OtherEvent},
    types::{Position3D, SparseSet},
};
use super::{Canvas, Layer, Object, render_unit::*};
use my_term::color::{BLACK, Background, Foreground, WHITE};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::{Arc, mpsc},
};

// ##################
// ## Type Aliases ##
// ##################
type Grid = SparseSet<RenderUnit>;
type DynRefList = Vec<Weak<RefCell<Object>>>;
pub type RenderQueue = mpsc::Sender<RenderSignal>;

// ############################
// ## Main Loop For Renderer ##
// ############################
pub fn render_thread(
    ctx: Context,
    mut canvas: Canvas,
    rx: mpsc::Receiver<RenderSignal>,
    event_tx: mpsc::Sender<Event>,
    lg: Arc<logging::Logger>,
) {
    #[allow(unused)]
    let mut force_refresh: std::time::Instant = std::time::Instant::now();
    #[allow(unused)]
    let tick_rate: std::time::Duration = std::time::Duration::from_millis(10);
    let mut bg_counter: usize = 1;
    let mut mg_counter: usize = 1;
    let mut fg_counter: usize = 1;
    let mut ui_counter: usize = 1;
    let mut free_bg: Vec<usize> = Vec::new();
    let mut free_mg: Vec<usize> = Vec::new();
    let mut free_fg: Vec<usize> = Vec::new();
    let mut free_ui: Vec<usize> = Vec::new();
    let mut background: Grid = SparseSet::new(1000);
    let mut middleground: Grid = SparseSet::new(1000);
    let mut foreground: Grid = SparseSet::new(1000);
    let mut ui: Grid = SparseSet::new(1000);
    let mut dynamics_list: DynRefList = Vec::new();
    let mut foreground_color: Foreground = Foreground::new(WHITE);
    let mut background_color: Background = Background::new(BLACK);
    let mut dirty: bool = true;
    let mut camera: Camera = Camera::new(canvas.width as u32, canvas.height as u32);
    let mut ui_camera: Camera = Camera::new(canvas.width as u32, canvas.height as u32);

    // Main Loop
    while ctx.is_alive() {
        check_for_signals(
            &mut foreground,
            &mut middleground,
            &mut background,
            &mut ui,
            &mut dynamics_list,
            &mut fg_counter,
            &mut mg_counter,
            &mut bg_counter,
            &mut ui_counter,
            &mut free_fg,
            &mut free_mg,
            &mut free_bg,
            &mut free_ui,
            &rx,
            &event_tx,
            &mut dirty,
            &mut canvas,
            &mut foreground_color,
            &mut background_color,
            &mut camera,
            &mut ui_camera,
            lg.clone(),
        );

        // Managing Dynamic Sprites //
        clear_invalid_weak_refs(&mut dynamics_list, &mut dirty);
        update_dynamic_objects(&mut dynamics_list, &mut dirty);

        /*
        if force_refresh.elapsed() >= tick_rate {
            dirty = true;
            force_refresh = std::time::Instant::now();
        }
        */
        // Print State to Terminal Screen //
        if dirty {
            print(
                &background,
                &middleground,
                &foreground,
                &ui,
                &canvas,
                &foreground_color,
                &background_color,
                &camera,
                &ui_camera,
                lg.clone(),
            );
            dirty = false;
        }
    }
}

fn check_for_signals(
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    ui: &mut Grid,
    dyn_list: &mut DynRefList,
    fg_counter: &mut usize,
    mg_counter: &mut usize,
    bg_counter: &mut usize,
    ui_counter: &mut usize,
    free_fg: &mut Vec<usize>,
    free_mg: &mut Vec<usize>,
    free_bg: &mut Vec<usize>,
    free_ui: &mut Vec<usize>,
    rx: &mpsc::Receiver<RenderSignal>,
    event_tx: &mpsc::Sender<Event>,
    dirty: &mut bool,
    canvas: &mut Canvas,
    fg_color: &mut Foreground,
    bg_color: &mut Background,
    camera: &mut Camera,
    ui_camera: &mut Camera,
    _lg: Arc<logging::Logger>,
) {
    for (i, msg) in rx.try_iter().enumerate() {
        if i == 0 {
            *dirty = true;
        }
        dispatch_msg(
            msg,
            fg,
            mg,
            bg,
            ui,
            dyn_list,
            fg_counter,
            mg_counter,
            bg_counter,
            ui_counter,
            free_fg,
            free_mg,
            free_bg,
            free_ui,
            event_tx,
            canvas,
            fg_color,
            bg_color,
            camera,
            ui_camera,
            _lg.clone(),
        );
    }
}
fn dispatch_msg(
    msg: RenderSignal,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    ui: &mut Grid,
    dyn_list: &mut DynRefList,
    fg_counter: &mut usize,
    mg_counter: &mut usize,
    bg_counter: &mut usize,
    ui_counter: &mut usize,
    free_fg: &mut Vec<usize>,
    free_mg: &mut Vec<usize>,
    free_bg: &mut Vec<usize>,
    free_ui: &mut Vec<usize>,
    event_tx: &mpsc::Sender<Event>,
    canvas: &mut Canvas,
    fg_color: &mut Foreground,
    bg_color: &mut Background,
    camera: &mut Camera,
    ui_camera: &mut Camera,
    _lg: Arc<logging::Logger>,
) {
    match msg {
        RenderSignal::Batch(mut batch) => batch_msg(
            &mut batch, fg, mg, bg, ui, dyn_list, fg_counter, mg_counter, bg_counter, ui_counter,
            free_fg, free_mg, free_bg, free_ui, event_tx, canvas, fg_color, bg_color, camera,
            ui_camera, _lg,
        ),
        RenderSignal::Sequence(mut seq) => sequence_msg(
            &mut seq, fg, mg, bg, ui, dyn_list, fg_counter, mg_counter, bg_counter, ui_counter,
            free_fg, free_mg, free_bg, free_ui, event_tx, canvas, fg_color, bg_color, camera,
            ui_camera, _lg,
        ),
        RenderSignal::TermSizeChange(c, r) => term_size_change_msg(c, r, canvas, event_tx),
        RenderSignal::Insert(id_holder, new_obj) => insert(
            id_holder, new_obj, fg_counter, mg_counter, bg_counter, ui_counter, free_fg, free_mg,
            free_bg, free_ui, fg, mg, bg, ui, dyn_list, canvas,
        ),
        RenderSignal::Background(bg) => change_bg(bg, bg_color),
        RenderSignal::Foreground(fg) => change_fg(fg, fg_color),
        RenderSignal::Remove(key) => match key.as_ref() {
            RenderUnitId::Background(_) => remove(key, bg),
            RenderUnitId::Middleground(_) => remove(key, mg),
            RenderUnitId::Foreground(_) => remove(key, fg),
            RenderUnitId::Ui(_) => remove(key, ui),
        },
        RenderSignal::Clear => clear_msg(fg, mg, bg, ui, dyn_list),
        RenderSignal::Redraw => {} // Used to mark display as dirty
        RenderSignal::Move(id, pos) => move_object(id, pos, fg, mg, bg, ui),
        RenderSignal::MoveLayer(id, layer) => move_layer(id, layer, fg, mg, bg, ui),
        RenderSignal::MoveCamera(pos) => camera.shift(pos.x, pos.y, pos.z),
        RenderSignal::PageUI(delta) => ui_camera.shift(0, (ui_camera.height() as i32) * delta, 0),
        RenderSignal::ScrollUI(delta) => {
            let _ = _lg.write(
                logging::LogLevel::Info,
                format!("scrolling camera by: {}", delta),
            );
            ui_camera.shift(0, delta, 0)
        }
        RenderSignal::ShiftUI(delta) => {
            let _ = _lg.write(
                logging::LogLevel::Info,
                format!("shifting camera by: {}", delta),
            );
            ui_camera.shift(delta, 0, 0);
        }
        RenderSignal::SetCamera(pos) => camera.set_pos(pos.x, pos.y, pos.z),
        RenderSignal::Update(id, obj) => update_object(id, obj, fg, mg, bg, ui, canvas),
    }
}

// ! TODO: Need to add camera object to the renderer so that I can make sure that all the things are where they need to be
fn print(
    bg: &Grid,
    mg: &Grid,
    fg: &Grid,
    ui: &Grid,
    can: &Canvas,
    fg_col: &Foreground,
    bg_col: &Background,
    cam: &Camera,
    ui_cam: &Camera,
    _lg: Arc<logging::Logger>,
) {
    let mut count = 0;
    let mut stdout = std::io::stdout();
    let col: String = format!("{}{}", &fg_col, &bg_col);
    let _ = write!(stdout, "{CLEAR_COLORS}\x1b[2J");
    let _ = stdout.flush();
    print_layer(&mut stdout, bg, cam, can, &col, &mut count);
    let _ = stdout.flush();
    print_layer(&mut stdout, mg, cam, can, &col, &mut count);
    let _ = stdout.flush();
    print_layer(&mut stdout, fg, cam, can, &col, &mut count);
    let _ = stdout.flush();
    print_layer(&mut stdout, ui, ui_cam, can, &col, &mut count);
    let _ = stdout.flush();
    let _ = write!(
        stdout,
        "\x1b[{};0f|ui_cam:{},{},{},{} | cam:{},{},{},{} | Objects Rendered {}                   |",
        can.height - 1,
        ui_cam.x(),
        ui_cam.y(),
        ui_cam.width(),
        ui_cam.height(),
        cam.x(),
        cam.y(),
        cam.width(),
        cam.height(),
        count
    );
    let _ = stdout.flush();
}

// #####################
// ## Print Functions ##
// #####################
//
fn print_layer(
    stream: &mut Stdout,
    g: &Grid,
    cam: &Camera,
    can: &Canvas,
    col: &str,
    count: &mut i32,
) {
    for (k, _) in g.all_keys() {
        g.get(*k)
            .unwrap()
            .object
            .borrow_mut()
            .draw(can, cam, stream, col);
        *count += 1;
    }
}

// ######################
// ## Helper Functions ##
// ######################
fn batch_msg(
    messages: &mut Vec<RenderSignal>,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    ui: &mut Grid,
    dyn_list: &mut DynRefList,
    fg_counter: &mut usize,
    mg_counter: &mut usize,
    bg_counter: &mut usize,
    ui_counter: &mut usize,
    free_fg: &mut Vec<usize>,
    free_mg: &mut Vec<usize>,
    free_bg: &mut Vec<usize>,
    free_ui: &mut Vec<usize>,
    event_tx: &mpsc::Sender<Event>,
    canvas: &mut Canvas,
    fg_color: &mut Foreground,
    bg_color: &mut Background,
    camera: &mut Camera,
    ui_camera: &mut Camera,
    _lg: Arc<logging::Logger>,
) {
    while messages.len() > 0 {
        dispatch_msg(
            messages.swap_remove(0),
            fg,
            mg,
            bg,
            ui,
            dyn_list,
            fg_counter,
            mg_counter,
            bg_counter,
            ui_counter,
            free_fg,
            free_mg,
            free_bg,
            free_ui,
            event_tx,
            canvas,
            fg_color,
            bg_color,
            camera,
            ui_camera,
            _lg.clone(),
        );
    }
}

fn sequence_msg(
    messages: &mut Vec<RenderSignal>,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    ui: &mut Grid,
    dyn_list: &mut DynRefList,
    fg_counter: &mut usize,
    mg_counter: &mut usize,
    bg_counter: &mut usize,
    ui_counter: &mut usize,
    free_fg: &mut Vec<usize>,
    free_mg: &mut Vec<usize>,
    free_bg: &mut Vec<usize>,
    free_ui: &mut Vec<usize>,
    event_tx: &mpsc::Sender<Event>,
    canvas: &mut Canvas,
    fg_color: &mut Foreground,
    bg_color: &mut Background,
    camera: &mut Camera,
    ui_camera: &mut Camera,
    _lg: Arc<logging::Logger>,
) {
    while messages.len() > 0 {
        dispatch_msg(
            messages.remove(0),
            fg,
            mg,
            bg,
            ui,
            dyn_list,
            fg_counter,
            mg_counter,
            bg_counter,
            ui_counter,
            free_fg,
            free_mg,
            free_bg,
            free_ui,
            event_tx,
            canvas,
            fg_color,
            bg_color,
            camera,
            ui_camera,
            _lg.clone(),
        );
    }
}

fn change_bg(new: Background, bg: &mut Background) {
    *bg = new;
}

fn change_fg(new: Foreground, fg: &mut Foreground) {
    *fg = new;
}

fn term_size_change_msg(cols: u32, rows: u32, canvas: &mut Canvas, event_tx: &mpsc::Sender<Event>) {
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
    data: ObjectData,
    fg_counter: &mut usize,
    mg_counter: &mut usize,
    bg_counter: &mut usize,
    ui_counter: &mut usize,
    free_fg: &mut Vec<usize>,
    free_mg: &mut Vec<usize>,
    free_bg: &mut Vec<usize>,
    free_ui: &mut Vec<usize>,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    ui: &mut Grid,
    dyn_list: &mut DynRefList,
    canvas: &Canvas,
) {
    let ins = |set: &mut SparseSet<RenderUnit>,
               id_counter: &mut usize,
               free_ids: &mut Vec<usize>,
               id_holder: Arc<RenderUnitId>,
               data: ObjectData,
               canvas: &Canvas| {
        let new_unit: RenderUnit = RenderUnit {
            id: id_holder,
            object: Rc::new(RefCell::new(Object::from_data(data, canvas))),
        };
        new_unit.id.store(if free_ids.len() > 0 {
            free_ids.swap_remove(0)
        } else {
            *id_counter += 1;
            *id_counter - 1
        });
        set.insert(new_unit.id.load(), new_unit);
    };
    let ins_dyn = |dyn_list: &mut DynRefList,
                   set: &mut SparseSet<RenderUnit>,
                   id_counter: &mut usize,
                   free_ids: &mut Vec<usize>,
                   id_holder: Arc<RenderUnitId>,
                   data: ObjectData,
                   canvas: &Canvas| {
        let new_unit: RenderUnit = RenderUnit {
            id: id_holder,
            object: Rc::new(RefCell::new(Object::from_data(data, canvas))),
        };
        new_unit.id.store(if free_ids.len() > 0 {
            free_ids.swap_remove(0)
        } else {
            *id_counter += 1;
            *id_counter - 1
        });
        dyn_list.push(Rc::downgrade(&new_unit.object));
        set.insert(new_unit.id.load(), new_unit);
    };

    let is_dyn = match &data {
        ObjectData::Sprite { pos: _, glyph } => match glyph {
            GlyphType::Multi {
                frames: _,
                tick_rate: _,
            } => true,
            GlyphType::Single(_) => false,
        },
        ObjectData::Text {
            pos: _,
            data,
            style: _,
        } => match data {
            TextType::Multi {
                frames: _,
                tick_rate: _,
            } => true,
            TextType::Single(_) => false,
        },
    };

    if is_dyn {
        match id_holder.layer() {
            Layer::Background => {
                ins_dyn(dyn_list, bg, bg_counter, free_bg, id_holder, data, canvas)
            }
            Layer::Middleground => {
                ins_dyn(dyn_list, mg, mg_counter, free_mg, id_holder, data, canvas)
            }
            Layer::Foreground => {
                ins_dyn(dyn_list, fg, fg_counter, free_fg, id_holder, data, canvas)
            }
            Layer::Ui => ins_dyn(dyn_list, ui, ui_counter, free_ui, id_holder, data, canvas),
        }
    } else {
        match id_holder.layer() {
            Layer::Background => ins(bg, bg_counter, free_bg, id_holder, data, canvas),
            Layer::Middleground => ins(mg, mg_counter, free_mg, id_holder, data, canvas),
            Layer::Foreground => ins(fg, fg_counter, free_fg, id_holder, data, canvas),
            Layer::Ui => ins(ui, ui_counter, free_ui, id_holder, data, canvas),
        }
    }
}

fn remove(key: Arc<RenderUnitId>, list: &mut Grid) {
    list.remove(key.load());
}

fn clear_msg(
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    ui: &mut Grid,
    dynamics_list: &mut DynRefList,
) {
    print!("\x1b[2J");
    fg.clear();
    mg.clear();
    bg.clear();
    ui.clear();
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
        if let Some(arc) = each.upgrade() {
            let mut each = arc.borrow_mut();
            if each.update() {
                *dirty = true;
            }
        } else {
            // Log that something went wrong
        }
    }
}

fn move_object(
    id: Arc<RenderUnitId>,
    pos: Position3D<i32>,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    ui: &mut Grid,
) {
    let (id, layer) = (id.load(), id.layer());
    match layer {
        Layer::Background => bg.get(id).unwrap(),
        Layer::Middleground => mg.get(id).unwrap(),
        Layer::Foreground => fg.get(id).unwrap(),
        Layer::Ui => ui.get(id).unwrap(),
    }
    .object
    .borrow_mut()
    .move_pos(pos);
}

fn move_layer(
    id: Arc<RenderUnitId>,
    layer: Layer,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    ui: &mut Grid,
) {
    let (id, current_layer) = (id.load(), id.layer());
    let mut id = id;
    let mut changed = false;
    let val = match current_layer {
        Layer::Background => bg.remove(id).unwrap(),
        Layer::Middleground => mg.remove(id).unwrap(),
        Layer::Foreground => fg.remove(id).unwrap(),
        Layer::Ui => ui.remove(id).unwrap(),
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
        Layer::Ui => {
            while ui.is_filled(id) {
                id += 1;
                if !changed {
                    changed = true;
                }
            }
            if changed {
                val.id.store(id);
                ui.insert(id, val)
            }
        }
    }
}

fn update_object(
    id: Arc<RenderUnitId>,
    data: ObjectData,
    fg: &mut Grid,
    mg: &mut Grid,
    bg: &mut Grid,
    ui: &mut Grid,
    canvas: &Canvas,
) {
    match id.layer() {
        Layer::Background => {
            if let Some(unit) = bg.get(id.load()) {
                *unit.object.borrow_mut() = Object::from_data(data, canvas);
            } else {
                // Log that there was a problem
            }
        }
        Layer::Middleground => {
            if let Some(unit) = mg.get(id.load()) {
                *unit.object.borrow_mut() = Object::from_data(data, canvas);
            } else {
                // Log that there was a problem
            }
        }
        Layer::Foreground => {
            if let Some(unit) = fg.get(id.load()) {
                *unit.object.borrow_mut() = Object::from_data(data, canvas);
            } else {
                // Log that there was a problem
            }
        }
        Layer::Ui => {
            if let Some(unit) = ui.get(id.load()) {
                *unit.object.borrow_mut() = Object::from_data(data, canvas);
            } else {
                // Log that there was a problem
            }
        }
    }
}
