use crate::engine::types::StoreItem;
use std::{cmp::Ordering, hash::Hash};

#[derive(Clone, PartialOrd, Debug, serde::Serialize, serde::Deserialize)]
pub struct Stat {
    name: String,
    max: f32,
    current: f32,
}

impl StoreItem for Stat {
    type Key = String;
    fn key(&self) -> Self::Key {
        self.name.clone()
    }
}

impl Stat {
    pub fn canon_max(&self) -> u32 {
        if self.max.is_nan() {
            f32::NAN.to_bits()
        } else if self.max == 0.0 {
            0
        } else {
            self.max.to_bits()
        }
    }
    pub fn canon_current(&self) -> u32 {
        if self.current.is_nan() {
            f32::NAN.to_bits()
        } else if self.current == 0.0 {
            0
        } else {
            self.current.to_bits()
        }
    }
}

impl PartialEq for Stat {
    fn eq(&self, other: &Self) -> bool {
        let sm: u32; // self max
        let sc: u32; // self current
        let om: u32; // other max
        let oc: u32; // other current

        self.canon_max() == other.canon_max() && self.canon_current() == other.canon_current()
    }
}
impl Eq for Stat {}
impl Ord for Stat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.canon_max().cmp(&other.canon_max()) {
            Ordering::Equal => self.canon_current().cmp(&other.canon_current()),
            ord => ord,
        }
    }
}

impl Hash for Stat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let m: u32;
        let c: u32;

        if self.max.is_nan() {
            m = f32::NAN.to_bits();
        } else if self.max == 0.0 {
            m = 0;
        } else {
            m = self.max.to_bits();
        }

        if self.current.is_nan() {
            c = f32::NAN.to_bits();
        } else if self.current == 0.0 {
            c = 0;
        } else {
            c = self.current.to_bits();
        }

        state.write_u32(m);
        state.write_u32(c);
    }

    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for each in data.iter() {
            each.hash(state);
        }
    }
}

impl Stat {
    pub fn new(name: String, val: f32) -> Self {
        Self {
            name: name,
            max: val,
            current: val,
        }
    }

    pub fn max_increase(&mut self, val: f32) {
        self.max += val;
    }
    pub fn max_decrease(&mut self, val: f32) {
        self.max -= val;
    }
    pub fn increase(&mut self, val: f32) {
        if self.current + val > self.max {
            self.current = self.max;
        } else {
            self.current += val;
        }
    }
    pub fn increase_overload(&mut self, val: f32) {
        self.current += val;
    }
    pub fn decrease(&mut self, val: f32) {
        if self.current - val < 0.0 {
            self.current = 0.0;
        } else {
            self.current -= val;
        }
    }
    pub fn decrease_overload(&mut self, val: f32) {
        self.current -= val;
    }
}
