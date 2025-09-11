use crate::app;

pub type Action<S> = fn(&mut S, &mut app::core::Context) -> Result<(), app::error::State>;