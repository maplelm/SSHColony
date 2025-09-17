use super::{
    enums::{Msg, Object},
    structs::Canvas,
};
use crate::engine::types::*;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::mpsc,
};

pub fn msg_dispatch(
    msg: Msg,
    canvas: &Canvas,
    prefix: &mut String,
    suffix: &mut String,
    objects: &mut Vec<Option<Rc<RefCell<Object>>>>,
    dynamics: &mut Vec<Weak<RefCell<Object>>>,
) {
    match msg {
        Msg::Batch(b) => {
            for each in b {
                msg_dispatch(each, canvas, prefix, suffix, objects, dynamics);
            }
        }
        Msg::Insert(pos, obj) => match obj {
            Object::Static { .. } => {
                objects[pos.y * canvas.width + pos.x] = Some(Rc::new(RefCell::new(obj)))
            }
            Object::Dynamic { .. } => {
                let val = Rc::new(RefCell::new(obj));
                dynamics.push(Rc::downgrade(&val));
                objects[pos.y * canvas.width + pos.x] = Some(val);
            }
        },
        Msg::Prefix(p) => *prefix = p,
        Msg::Suffix(s) => *suffix = s,
        Msg::InsertRange { start, end, object } => {
            let start_pos = start.y * canvas.width + start.x;
            match object {
                Object::Static { .. } => {
                    for y in 0..(start.y as i32 - end.y as i32).abs() {
                        for x in 0..(start.x as i32 - end.x as i32).abs() {
                            objects[start_pos + (y as usize * canvas.width + x as usize)] =
                                Some(Rc::new(RefCell::new(object.clone())));
                        }
                    }
                }
                Object::Dynamic { .. } => {
                    for y in 0..(start.y as i32 - end.y as i32).abs() {
                        for x in 0..(start.x as i32 - end.x as i32).abs() {
                            let val = Rc::new(RefCell::new(object.clone()));
                            dynamics.push(Rc::downgrade(&val));
                            objects[start_pos + (y as usize * canvas.width + x as usize)] =
                                Some(val);
                        }
                    }
                }
            }
        }
        Msg::InsertText {
            pos,
            text,
            prefix,
            suffix,
        } => {
            //Msg::InsertText(pos, text, prefix, suffix) => {
            let mut y: usize = pos.y;
            let mut x: usize = pos.x;
            for each in text.chars() {
                if !each.is_ascii_graphic() || each == ' ' || each == '\n' {
                    objects[y * canvas.width + x] = None;
                } else {
                    let mut sprite = String::from(each);
                    if let Some(prefix) = &prefix {
                        sprite.insert_str(0, prefix);
                    }
                    if let Some(suffix) = &suffix {
                        sprite.push_str(suffix);
                    }
                    objects[y * canvas.width + x] =
                        Some(Rc::new(RefCell::new(Object::new_static(sprite).unwrap())))
                }
                if each == '\n' {
                    y += 1;
                    x = pos.x;
                } else {
                    x += 1;
                }
            }
        }
        Msg::Remove(pos) => {
            objects[pos.y as usize * canvas.width + pos.x] = None;
        }
        Msg::RemoveRange(start, end) => {
            let start_pos: usize = start.y * canvas.width + start.x;
            for y in 0..(start.y as i32 - end.y as i32).abs() {
                for x in 0..(start.x as i32 - end.y as i32).abs() {
                    objects[start_pos + (y as usize * canvas.width + x as usize)] = None;
                }
            }
        }
        Msg::Swap(a, b) => {
            objects.swap(a.y * canvas.width + a.x, b.y * canvas.width + b.x);
        }
        Msg::Clear => {
            objects.fill(None);
            dynamics.clear();
        }
        _ => {
            todo!()
        }
    }
}

pub fn insert_text(
    x: usize,
    y: usize,
    text: String,
    sender: &mpsc::Sender<Msg>,
) -> Result<(), mpsc::SendError<Msg>> {
    sender.send(Msg::InsertText {
        pos: Position::new(x, y),
        text: text,
        prefix: None,
        suffix: None,
    })
}
