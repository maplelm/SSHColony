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

use crate::{
    engine::{
        render,
        traits::Storeable,
        types::{Position, Position3D, Store},
    },
    game::types::{inventory::Inventory, stat::Stat, stat::StatTemplate},
};
use std::sync::{Arc, Weak, atomic::AtomicUsize};
use std::{cmp::Ordering, collections::HashMap, hash::Hash};

////////////
// Consts //
////////////
const ENTITY_PASSABLE_FLAG: u8 = 0b00000001;
const ENTITY_STORABLE_FLAG: u8 = 0b00000010;

/////////////
// Structs //
/////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Common {
    flags: u8,
    #[serde(skip, default = "Common::default_render_id")]
    render_id: Weak<AtomicUsize>,
    pub position: Position3D<usize>,
    pub inventory: Option<Inventory>,
    pub stats: HashMap<String, Stat>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Kind {
    Creature {
        kind: CreatureType,
        state: CreatureState,
    },
    Object {
        kind: ObjectType,
        state: ObjectState,
    },
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    pub common: Common,
    pub kind: Kind,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Template {
    pub passable: bool,
    pub storable: bool,
    pub name: String,
    pub max_inventory_weight: Option<u32>,
    pub base_stats: HashMap<String, StatTemplate>,
    pub kind: Kind,
}

////////////////////
// Implementation //
////////////////////
impl Common {
    fn default_render_id() -> Weak<AtomicUsize> {
        Weak::new()
    }
}

impl Entity {
    pub fn from_template(temp: Template) -> Self {
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
                render_id: Weak::new(),
                position: Position3D { x: 0, y: 0, z: 0 },
                inventory: if let Some(weight) = temp.max_inventory_weight {
                    Inventory::new(weight)
                } else {
                    None
                },
                stats: stats,
            },
            kind: temp.kind,
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

impl Storeable for Template {
    type Key = String;
    fn key(&self) -> Self::Key {
        self.name.clone()
    }
}

///////////
// Enums //
///////////

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CreatureType {
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
pub enum ObjectType {
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
