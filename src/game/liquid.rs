#![allow(dead_code)]
use super::material::Material;

pub struct Liquid {
    level: u8,
    kind: Kind,
}

pub enum Kind {
    Water,
    Lava,
    Other(Material),
}
