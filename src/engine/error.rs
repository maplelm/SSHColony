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


use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorKind {
    #[serde(skip)]
    Io(std::io::ErrorKind),
    Network,
    InvalidData,
    RebuildRequired,
    NotFound,
    Context,
    ContextDead,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(subkind) => write!(f, "IO({})", subkind),
            Self::Network => write!(f, "Network"),
            Self::InvalidData => write!(f, "Invalid Data"),
            Self::RebuildRequired => write!(f, "Rebuild Required"),
            Self::NotFound => write!(f, "Not Found"),
            Self::Context => write!(f, "Context"),
            Self::ContextDead => write!(f, "Context Dead"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    msg: String,
    kind: ErrorKind,
    #[serde(skip)]
    source: Option<Box<dyn std::error::Error + 'static>>,
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_deref()
    }
}

impl Error {
    pub fn new(msg: impl Into<String>, kind: ErrorKind) -> Self {
        Self {
            msg: msg.into(),
            kind,
            source: None
        }
    }

    pub fn from<E>(err: E, msg: impl Into<String>, kind: ErrorKind) -> Self 
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self {
            msg: msg.into(),
            kind,
            source: Some(Box::new(err)),
        }
    }
}