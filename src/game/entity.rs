use super::{Stat, enums};
use crate::engine::{
    render,
    types::{Position, Position3D, Store, StoreItem},
};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

const ENTITY_PASSABLE_FLAG: u8 = 0b00000001;
const ENTITY_STORABLE_FLAG: u8 = 0b00000010;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    pub common: Common,
    pub kind: Kind,
}

impl StoreItem for Entity {
    type Key = String;
    fn key(&self) -> Self::Key {
        self.common.name.clone()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Common {
    pub flags: u8,
    pub name: String,
    #[serde(skip, default="pos_zero")]
    pub position: Position3D<usize>,
    #[serde(skip, default="vpos_none")]
    pub visual_position: Option<Position<usize>>,
    #[serde(skip, default="inventory_zero")]
    pub inventory: Vec<usize>,
    pub max_inventory_weight: u32, // items should have a wait based on volume and density
    pub stats: Store<Stat>,
}

fn inventory_zero() -> Vec<usize> {
    vec![]
}
fn pos_zero() -> Position3D<usize> {
    Position3D { x: 0, y: 0, z: 0}
}

fn vpos_none() -> Option<Position<usize>> {
    None
} 

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Kind {
    Creature {
        kind: enums::Creatures,
        state: enums::CreatureState,
    },
    Object {
        kind: enums::Objects,
        state: enums::ObjectState,
    },
}

impl Entity {
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
