#![allow(dead_code, unused)]
use super::entity::Entity;
use super::material::Material;
use super::tile::{self, Tile};
use crate::engine::{
    self, render,
    types::{Position3D, Store},
};
use serde;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct World {
    pub avg_temp: f32,
    pub avg_height: f32,
    pub sea_level: f32,
    pub world_size: Position3D<usize>,

    #[serde(skip, default="world_init_materials")]
    pub materials: Store<Material>,

    #[serde(skip, default="world_init_entities")]
    pub entities: Store<Entity>,

    #[serde(skip, default="world_init_sprites")]
    pub sprites: Store<render::Object>,

    #[serde(default)]
    pub active_entities: Vec<Entity>,

    #[serde(default)]
    pub active_tiles: Vec<Tile>,
}

fn world_init_sprites() -> Store<render::Object> {
    if let Some(objs) = Store::from_dir("./data/sprites/"){
        objs
    } else {
        panic!("failed to load entities from ./data/sprites/")
    }
}

fn world_init_entities() -> Store<Entity> {
    if let Some(ents) = Store::from_dir("./data/entities/"){
        ents
    } else {
        panic!("failed to load entities from ./data/entities/")
    }
}

fn world_init_materials() -> Store<Material> {
    if let Some(mats) = Store::from_dir("./data/materials/") {
        mats
    } else {
        panic!("failed to load materials from ./data/materials/")
    }
}

impl World {
    pub fn new(
        x: usize,
        y: usize,
        z: usize,
        temp: f32,
        height: f32,
        sea: f32,
        mat_dir: &str,
        ent_dir: &str,
        spr_dir: &str,
    ) -> Self {
        let mut world = Self {
            world_size: Position3D::new(x, y, z),
            avg_temp: temp,
            avg_height: height,
            sea_level: sea,
            materials: Store::from_dir(mat_dir).unwrap(),
            entities: Store::from_dir(ent_dir).unwrap(),
            sprites: Store::from_dir(spr_dir).unwrap(),
            active_entities: vec![],
            active_tiles: Vec::with_capacity(x * y * z),
        };
        return world;
    }

    pub fn tile_at(&mut self, x: usize, y: usize, z: usize) -> Option<&mut Tile> {
        self.active_tiles
            .get_mut(z * (self.world_size.y * self.world_size.x) + (y * self.world_size.x) + x)
    }

    pub fn generate(&mut self, seed: Option<usize>) -> Result<(), engine::Error> {
        for index in 0..(self.world_size.x * self.world_size.y * self.world_size.z) {
            let pos = linear_to_3d(self, index);
            if pos.z > 0 {
                self.active_tiles
                    .push(Tile::new(0, None, None, tile::Shape::OpenSpace, 0));
            } else {
                self.active_tiles
                    .push(Tile::new(0, None, None, tile::Shape::Floor, 0));
            }
        }
        Ok(())
    }
}

fn linear_to_3d(w: &World, mut index: usize) -> Position3D<usize> {
    let z = index / (w.world_size.y * w.world_size.x);
    index = index % (w.world_size.y * w.world_size.x);
    let y = index / w.world_size.x;
    let x = index % w.world_size.x;
    Position3D::new(x, y, z)
}
