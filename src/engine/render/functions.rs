use super::{
    enums::{Msg, Object},
    structs::Canvas,
    super::{
        traits::Scene, enums::Signal, input::Event, types::*
    }
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::mpsc,
};

pub fn clear(tx: &mpsc::Sender<Msg>) -> Result<(), mpsc::SendError<Msg>> {
    tx.send(Msg::Clear)
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
