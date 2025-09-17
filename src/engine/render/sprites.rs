use super::color;
use std::time::{Duration};

use super::Object;

pub fn dwarf_object() -> Object {
    Object::new_static(format!("{}D", color::ISO_WHITE_FOREGROUND)).unwrap()
}

pub fn hurt_dwarf_object(millis: u64) -> Object {
    Object::new_dynamic(vec![
            format!("{}D", color::ISO_RED_FOREGROUND),
            format!("{}D", color::ISO_WHITE_FOREGROUND),
        ], Duration::from_millis(millis)).unwrap()
}