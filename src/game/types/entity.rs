
use crate::{
    engine::{
        traits::Storeable,
        render,
        types::{Position, Position3D, Store},
    },
    game::types::{Inventory, Stat, StatTemplate},
};
use std::{cmp::Ordering, collections::HashMap, hash::Hash};

pub type EntityID = u32;

const ENTITY_PASSABLE_FLAG: u8 = 0b00000001;
const ENTITY_STORABLE_FLAG: u8 = 0b00000010;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Common {
    pub flags: u8,
    pub position: Position3D<usize>,
    pub visual_position: Option<Position<usize>>,
    pub inventory: Option<Inventory>,
    pub stats: HashMap<String, Stat>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Kind {
    Creature {
        kind: Creatures,
        state: CreatureState,
    },
    Object {
        kind: Objects,
        state: ObjectState,
    },
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    pub common: Common,
    pub kind: Kind,
}

impl Entity {

    pub fn from_template(temp: EntityTemplate) -> Self {
        let mut flags = 0;
        if temp.passable {
            flags |= ENTITY_PASSABLE_FLAG;
        }
        if temp.storable {
            flags |= ENTITY_STORABLE_FLAG;
        }

        let mut stats: HashMap<String, Stat> = HashMap::new();
        for (key, val) in temp.base_stats {
            stats.insert(key, Stat::from_template(val));
        }

        Self {
            common: Common { 
                flags: flags,
                position: Position3D { x: 0, y: 0, z: 0 },
                visual_position: None,
                inventory: if let Some(weight) = temp.max_inventory_weight {Inventory::new(weight)} else {None},
                stats: stats, 
            },
            kind: temp.kind
        }
    }

    pub fn is_passable(&self) -> bool {
        ENTITY_PASSABLE_FLAG & self.common.flags > 0
    }
    pub fn is_storable(&self) -> bool {
        ENTITY_STORABLE_FLAG & self.common.flags > 0
    }

    pub fn set_passable(&mut self, val: bool) {
        if val {
            self.common.flags |= ENTITY_PASSABLE_FLAG;
        } else {
            self.common.flags &= !ENTITY_PASSABLE_FLAG;
        }
    }

    pub fn set_storable(&mut self, val: bool) {
        if val {
            self.common.flags |= ENTITY_STORABLE_FLAG;
        } else {
            self.common.flags &= !ENTITY_STORABLE_FLAG;
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Creatures {
    Dwarf,
    Human,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CreatureState {
    Idle,
    Combat,
    Moving,
    Dead,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Objects {
    Door,
    Bin,
    Chair,
    Table,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ObjectState {
    Normal,
    Damaged,
    Broken,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EntityTemplate {
    pub passable: bool,
    pub storable: bool,
    pub name: String,
    pub max_inventory_weight: Option<u32>,
    pub base_stats: HashMap<String, StatTemplate>,
    pub kind: Kind,
}

impl Storeable for EntityTemplate {
    type Key = String;
    fn key(&self) -> Self::Key {
        self.name.clone()
    }
}
