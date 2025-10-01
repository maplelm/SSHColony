#![allow(dead_code, unused)]
use super::entity::{Entity, EntityTemplate};
use super::material::Material;
use super::tile::{self, Tile};
use crate::engine::Error;
use crate::engine::{
    self,
    render::{self, Object},
    types::{File, Position3D, SparseSet, Store},
};
use serde;
use std::io::Read;
use std::{fs, io::Write};

pub const MAX_MATERIALS: usize = 10000; //10K
pub const MAX_ENTITIES: usize = 10000000; // 10M

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WorldSave {
    name: String,
    pub avg_temp: f32,
    pub avg_height: f32,
    pub sea_level: f32,
    pub world_size: Position3D<usize>,
    pub materials: SparseSet<Material>,
    pub entities: SparseSet<Entity>,
    pub tiles: Vec<Tile>,
}

impl WorldSave {
    pub fn from_world(w: &World) -> WorldSave {
        Self {
            name: w.name.clone(),
            avg_temp: w.avg_temp,
            avg_height: w.avg_height,
            sea_level: w.sea_level,
            world_size: w.world_size,
            materials: w.materials.clone(),
            entities: w.entities.clone(),
            tiles: w.tiles.clone(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct World {
    name: String,
    pub avg_temp: f32,
    pub avg_height: f32,
    pub sea_level: f32,
    pub world_size: Position3D<usize>,
    #[serde(default = "world_init_materials")]
    pub material_templates: Store<Material>,
    #[serde(default = "world_init_entitytemplate")]
    pub entity_templates: Store<EntityTemplate>,
    pub materials: SparseSet<Material>,
    pub entities: SparseSet<Entity>,
    pub tiles: SparseSet<Tile>,
}

fn world_init_entitytemplate() -> Store<EntityTemplate> {
    match Store::from_dir("./data/entities/") {
        Err(e) => panic!("world_init_entitytemplate: {e}"),
        Ok(s) => s,
    }
}

fn world_init_materials() -> Store<Material> {
    match Store::from_dir("./data/materials/") {
        Err(e) => panic!("world_init_materials {e}"),
        Ok(s) => s,
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
    ) -> Result<Self, Error> {
        let mut materials: Store<Material>;
        match Store::<Material>::from_dir(mat_dir) {
            Err(e) => return Err(Error::IO(e)),
            Ok(m) => materials = m,
        }
        let mut entities: Store<EntityTemplate>;
        match Store::<EntityTemplate>::from_dir(ent_dir) {
            Err(e) => return Err(Error::IO(e)),
            Ok(e) => entities = e,
        }
        let mut sprites: Store<Object>;
        match Store::<Object>::from_dir(spr_dir) {
            Err(e) => return Err(Error::IO(e)),
            Ok(o) => sprites = o,
        }
        let mut world = Self {
            name: name,
            world_size: Position3D::new(x, y, z),
            avg_temp: temp,
            avg_height: height,
            sea_level: sea,
            material_templates: materials,
            entity_templates: entities,
            sprite_templates: sprites,
            materials: vec![],
            entities: vec![],
            tiles: Vec::with_capacity(x * y * z),
        };
        return Ok(world);
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
            let pos = self.index_to_pos(index);
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
        match File::open(&loc, false) {
            Ok(mut f) => {
                bincode::serde::encode_into_writer(save_data, f, bincode::config::standard());
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn index_to_pos(&self, mut index: usize) -> Position3D<usize> {
        let z = index / (self.world_size.y * self.world_size.x);
        index = index % (self.world_size.y * self.world_size.x);
        let y = index / self.world_size.x;
        let x = index % self.world_size.x;
        Position3D::new(x, y, z)
    }
}

