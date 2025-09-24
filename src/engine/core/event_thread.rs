use super::super::{input::{InputBuffer, poll_event, CLEAR_BUFFER,Event}, Context};
use std::{sync::mpsc, io::{stdin, Read}};

pub fn event_thread(ctx: Context, tx: mpsc::Sender<Event>) {
    while ctx.is_alive() {
        let mut buf: InputBuffer = CLEAR_BUFFER;
        match stdin().read(&mut buf) {
            Err(e) => {continue;}, // Log that there was an error
            Ok(_) => {},
        }
        let mut seq: &[u8] = &buf[0..1];
        if buf[0] == b'\x1b' {
            for (index, each) in buf.iter().enumerate() {
                if *each == b'\0' {
                    seq = &buf[0..index+1];
                    break;
                }
            }
        }
        match poll_event(&buf) {
            Some(event) => {
                if let Err(err) = tx.send(event) {
                    // log this and continue
                    continue;
                }
            }
            None => {} // log that data form stdin couldn't be transformed into an event by poll_event
        }
    }
    #[cfg(debug_assertions)]
    print!("Event Handling Thread has finished\r\n");
}