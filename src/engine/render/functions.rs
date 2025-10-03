#![deny(unused)]

use super::super::enums::RenderSignal;
use std::sync::mpsc;

pub fn clear(tx: &mpsc::Sender<RenderSignal>) -> Result<(), mpsc::SendError<RenderSignal>> {
    tx.send(RenderSignal::Clear)
}
