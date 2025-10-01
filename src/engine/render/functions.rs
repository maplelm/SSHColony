#![deny(unused)]

use super::enums::Msg;
use std::sync::mpsc;

pub fn clear(tx: &mpsc::Sender<Msg>) -> Result<(), mpsc::SendError<Msg>> {
    tx.send(Msg::Clear)
}
