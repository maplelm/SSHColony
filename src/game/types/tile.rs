#![allow(dead_code, unused)]

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tile {
    pub flags: u16,
    pub ocupency: Option<usize>,
    pub fluid: Option<(usize, usize)>, // material id, amount of liquid,
    pub shape: Shape,
    pub material: usize,
}

impl Tile {
    pub fn new(
        flag: u16,
        entity: Option<usize>,
        l: Option<(usize, usize)>,
        s: Shape,
        mat: usize,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StairDirection {
    Up,
    Down,
    Both,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
