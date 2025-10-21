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

use super::input::Event;
use std::error;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    SendEventError(std::sync::mpsc::SendError<Event>),
    InvalidData(Box<dyn error::Error>),
    RebuildRequired(Option<Box<dyn error::Error>>),
    NotFound(String, Option<Box<dyn error::Error>>),
    ContextError(String),
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "IO Error: {}", e),
            Error::InvalidData(d) => write!(f, "InvalidData: {}", d),
            Error::RebuildRequired(d) => {
                if d.is_some() {
                    write!(f, "Rebuild Required: {}", d.as_ref().unwrap())
                } else {
                    write!(f, "Rebuild Required")
                }
            }
            Error::NotFound(s, _) => write!(f, "Not Found: {}", s),
            Error::ContextError(ctx) => write!(f, "Context Error: {}", ctx),
            Error::SendEventError(e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IO(e) => Some(e),
            Error::ContextError(_) => None,
            Error::InvalidData(d) => d.source(),
            Error::RebuildRequired(d) => if d.is_some() {d.as_ref().unwrap().source()}else{None},
            Error::NotFound(s, inner) => if inner.is_some() {inner.as_ref().unwrap().source()}else{None},
            Error::SendEventError(e) => e.source(),
        }
    }
}

