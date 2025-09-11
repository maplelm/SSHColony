use std::error;
use super::input::Event;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    SendEventError(std::sync::mpsc::SendError<Event>),
    ContextError(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e)=> write!(f, "IO Error: {}", e),
            Error::ContextError(ctx) => write!(f, "Context Error: {}", ctx),
            Error::SendEventError(e) => e.fmt(f)
        }
    }
}

impl error::Error for Error {   
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IO(e) => Some(e),
            Error::ContextError(_) => None,
            Error::SendEventError(e) => e.source()
        }
    }
}