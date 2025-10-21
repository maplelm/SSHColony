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
