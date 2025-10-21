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
use super::super::{
    Context,
    input::{CLEAR_BUFFER, Event, InputBuffer, poll_event},
};
use chrono::Local;
use std::{
    io::{Read, Write, stdin},
    sync::mpsc,
};

pub fn event_thread(ctx: Context, tx: mpsc::Sender<Event>) {
    // Read the events comming from Stdin
    while ctx.is_alive() {
        let mut buf: InputBuffer = CLEAR_BUFFER;
        match stdin().read(&mut buf) {
            Err(e) => {}
            Ok(n) => {
                let mut seq: &[u8] = &buf[0..n];
                /*
                                if buf[0] == b'\x1b' {
                                    for (index, each) in buf.iter().enumerate() {
                                        if *each == b'\0' {
                                            seq = &buf[0..index + 1];
                                            break;
                                        }
                                    }
                                }
                */
                if seq[0] == b'\x1b' {
                    match poll_event(&seq) {
                        Some(event) => {
                            if let Err(err) = tx.send(event) {
                                // log this and continue
                                continue;
                            }
                        }
                        None => {} // log that data form stdin couldn't be transformed into an event by poll_event
                    }
                } else {
                    for (i, _) in seq.iter().enumerate() {
                        match poll_event(&seq[i..i + 1]) {
                            Some(e) => {
                                if let Err(err) = tx.send(e) {
                                    //logging
                                    continue;
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }
    }
    #[cfg(debug_assertions)]
    print!("Event Handling Thread has finished\r\n");
}
