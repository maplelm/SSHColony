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

#![allow(dead_code)]
use crate::engine::traits::Storeable;
use ron::de::from_str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub type MaterialID = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Common {
    pub name: String,
    // Physical Traits
    pub opacity: f32,
    pub density: f32,
    // Thermal Traits
    pub heat_capacity: f32,
    pub flammability: f32,
    // Electrical Traits
    pub electrical_conductivity: f32,
    pub electrical_impedance: f32,
    pub electrical_field: f32,
    pub electrical_charge: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    Solid {
        hardness: f32,
        malleability: f32,
        elasticity: f32,
        plasticity: f32,
        impact_resistance: f32,
        ductility: f32,
        melting_point: f32,
    },
    Liquid {
        viscosity: f32,
        boiling_point: f32,
    },
    Gas {
        compressibility: f32,
    },
    Plasma {
        ionization_fraction: f32,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Material {
    pub common: Common,
    pub state: State,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            common: Common {
                name: "default".to_string(),
                opacity: 0.0,
                density: 0.0,
                heat_capacity: 0.0,
                flammability: 0.0,
                electrical_conductivity: 0.0,
                electrical_impedance: 0.0,
                electrical_field: 0.0,
                electrical_charge: 0.0,
            },
            state: State::Solid {
                hardness: 0.0,
                malleability: 0.0,
                elasticity: 0.0,
                plasticity: 0.0,
                impact_resistance: 0.0,
                ductility: 0.0,
                melting_point: 0.0,
            },
        }
    }
}

impl Storeable for Material {
    type Key = String;
    fn key(&self) -> Self::Key {
        self.common.name.clone()
    }
}
