#![allow(dead_code, unused)]
use super::entity::{Entity, EntityTemplate};
use super::material::Material;
use super::tile::{self, Tile};
use crate::engine::{
    self, render,
    types::{Position3D, Store, File},
};
use serde;
use std::io::Read;
use std::{fs, io::Write};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WorldSave {
    name: String,
    pub avg_temp: f32,
    pub avg_height: f32,
    pub sea_level: f32,
    pub world_size: Position3D<usize>,
    pub materials: Vec<Material>,
    pub entities: Vec<Entity>,
    pub tiles: Vec<Tile>,
}

impl WorldSave {
    pub fn from_world(w: &World) -> WorldSave {
        Self { name: w.name.clone(), avg_temp: w.avg_temp, avg_height: w.avg_height, sea_level: w.sea_level, world_size: w.world_size, materials: w.materials.clone(), entities: w.entities.clone(), tiles: w.tiles.clone() }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct World {
    name: String,
    pub avg_temp: f32,
    pub avg_height: f32,
    pub sea_level: f32,
    pub world_size: Position3D<usize>,
    #[serde(default="world_init_materials")]
    pub material_templates: Store<Material>,
    #[serde(default="world_init_entitytemplate")]
    pub entity_templates: Store<EntityTemplate>,
    #[serde(default="world_init_sprites")]
    pub sprite_templates: Store<render::Object>,
    pub materials: Vec<Material>,
    pub entities: Vec<Entity>,
    pub tiles: Vec<Tile>,
}

fn world_init_sprites() -> Store<render::Object> {
    if let Some(objs) = Store::from_dir("./data/sprites/"){
        objs
    } else {
        panic!("failed to load entities from ./data/sprites/")
    }
}

fn world_init_entitytemplate() -> Store<EntityTemplate> {
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
        name: String,
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
            name: name,
            world_size: Position3D::new(x, y, z),
            avg_temp: temp,
            avg_height: height,
            sea_level: sea,
            material_templates: Store::from_dir(mat_dir).unwrap(),
            entity_templates: Store::from_dir(ent_dir).unwrap(),
            sprite_templates: Store::from_dir(spr_dir).unwrap(),
            materials: vec![],
            entities: vec![],
            tiles: Vec::with_capacity(x * y * z),
        };
        return world;
    }

    pub fn from_file(path: &str) -> Self {
        todo!()
    }

    pub fn tile_at(&mut self, x: usize, y: usize, z: usize) -> Option<&mut Tile> {
        self.tiles
            .get_mut(z * (self.world_size.y * self.world_size.x) + (y * self.world_size.x) + x)
    }

    pub fn generate(&mut self, seed: Option<usize>) -> Result<(), engine::Error> {
        for index in 0..(self.world_size.x * self.world_size.y * self.world_size.z) {
            let pos = linear_to_3d(self, index);
            if pos.z > 0 {
                self.tiles
                    .push(Tile::new(0, None, None, tile::Shape::OpenSpace, 0));
            } else {
                self.tiles
                    .push(Tile::new(0, None, None, tile::Shape::Floor, 0));
            }
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let loc = String::from("./saves/") + self.name.as_str() + ".world";
        let save_data = WorldSave::from_world(self);
        match File::open(&loc, false){
                Ok(mut f) => {
                    bincode::serde::encode_into_writer(save_data, f , bincode::config::standard());
                    Ok(())
                }
                Err(e) => Err(e)
            }
    }

}

fn linear_to_3d(w: &World, mut index: usize) -> Position3D<usize> {
    let z = index / (w.world_size.y * w.world_size.x);
    index = index % (w.world_size.y * w.world_size.x);
    let y = index / w.world_size.x;
    let x = index % w.world_size.x;
    Position3D::new(x, y, z)
}
