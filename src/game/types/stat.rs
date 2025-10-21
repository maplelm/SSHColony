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

use std::{cmp::Ordering, hash::Hash};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct StatTemplate {
    pub level: u32,
    pub level_scaling: u32,
}

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Debug, serde::Serialize, serde::Deserialize)]
pub struct Stat {
    pub current: i32,
    pub level_scaling: u32,
    pub rust: u32,
    pub level: u32,
    pub level_boost: i32,
}

impl Stat {

    pub fn from_template(temp: StatTemplate) -> Self {
        Self {
            current: (temp.level * temp.level_scaling) as i32,
            level_scaling: temp.level_scaling,
            rust: 0,
            level: temp.level,
            level_boost: 0
        }
    }

    pub fn new(val: i32, scaling: u32) -> Self {
        Self {
            current: val,
            level: 1,
            level_boost: 0,
            rust: 0,
            level_scaling: scaling,
        }
    }

    pub fn max_value(&self) -> i32 {
        (self.level as i32 + self.level_boost) * self.level_scaling as i32
    }

    pub fn level_up(&mut self, amount: u32) {
        self.level += amount;
    }

    pub fn boost(&mut self, amount: i32) {
        self.level_boost += amount;
    }

    pub fn increase_current(&mut self, val: i32) {
        if self.current + val >  self.max_value(){
            self.current = self.max_value();
        } else {
            self.current += val;
        }
    }
    pub fn increase_current_overload(&mut self, val: i32) {
        self.current += val;
    }
    pub fn decrease_current(&mut self, val: i32) {
        if self.current - val < 0 {
            self.current = 0;
        } else {
            self.current -= val;
        }
    }
    pub fn decrease_current_overload(&mut self, val: i32) {
        self.current -= val;
    }
}
