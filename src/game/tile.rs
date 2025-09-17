#![allow(dead_code, unused)]
use super::{
    core::{EntityID, MaterialID},
    liquid::{self, Liquid},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Tile {
    pub flags: u16,
    pub ocupency: Option<EntityID>,
    pub fluid: Option<MaterialID>,
    pub shape: Shape,
    pub material: MaterialID,
}

impl Tile {
    pub fn new(
        flag: u16,
        entity: Option<EntityID>,
        l: Option<MaterialID>,
        s: Shape,
        mat: MaterialID,
    ) -> Self {
        Self {
            flags: flag,
            ocupency: entity,
            fluid: l,
            shape: s,
            material: mat,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum StairDirection {
    Up,
    Down,
    Both,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Shape {
    Floor,
    Wall,  // refined with a nice surface
    Block, // Nature untouched tile of solid material
    Ramp,
    Window,
    Fortification,
    OpenSpace,
    Stairs(StairDirection),
}
