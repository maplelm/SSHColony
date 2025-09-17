use std::io::Write;
use std::ops::{Add, Sub, Mul, Div};
use std::collections::HashMap;
use std::fs::{self, File, read_dir};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::hash::Hash;
use ron;

use crate::engine;

#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Position<T: Numeric> {
    pub x: T,
    pub y: T
}

impl<T: Numeric> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Position3D<T: Numeric> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Numeric> Position3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }
}

pub trait Numeric:  
    Copy
    + PartialEq     // Allow ==, !=
    + PartialOrd    // Allow >=, <=, ...
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{}

impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for f32 {}
impl Numeric for f64 {}
impl Numeric for usize {}
impl Numeric for isize {}


pub trait StoreItem: for<'de> serde::Deserialize<'de> + serde::Serialize{
    type Key: serde::Serialize + for<'de> serde::Deserialize<'de> + Eq + Hash + Clone;
    fn key(&self) -> Self::Key;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "T: StoreItem")]
pub struct Store<T: StoreItem>
where T::Key: Eq + std::hash::Hash, {
    matrix: Vec<T>,
    #[serde(skip, default="HashMap::new")]
    index_map: HashMap<T::Key, usize>
}

impl<T: StoreItem> Default for Store<T>
where T::Key: Eq + std::hash::Hash, {
    fn default() -> Self {
       Self { matrix: vec![], index_map: HashMap::new() } 
    }
}

impl<T: StoreItem> Store<T> 
where T::Key: Eq + std::hash::Hash, {
    pub fn rebuild(&mut self) {
        self.index_map.clear();
        for (i, each ) in self.matrix.iter().enumerate() {
            self.index_map.insert(each.key(), i);
        }
    }
    pub fn from_dir(dir: &str) -> Option<Store<T>> {
        let mut dir = read_dir(dir).ok();
        if dir.is_none() {
            return None;
        }
        let mut store = Store::default();
        while let Some(Ok(entry)) = dir.as_mut().unwrap().next() {
            if let Ok(t) = entry.file_type() && t.is_file() {
                let pathb = entry.path();
                let path = pathb.to_str();
                let nameb = entry.file_name();
                let name = nameb.to_str();
                if path.is_none() || name.is_none() {
                    // log that there is a malformed file path
                    continue;
                }
                if let Ok(file) = File::open(path.unwrap().to_string()+name.unwrap()){
                        let m: Option<Vec<T>> = if let Ok(data) = ron::de::from_reader(file) { Some(data) } else {None};
                        if m.is_none() {
                            // Log about the malformed data in file
                            continue;
                        }
                        let items: Vec<T> = m.unwrap();
                        store.matrix.extend(items);
                }
            }
            
        }
        for (i, each) in store.matrix.iter().enumerate() {
            store.index_map.insert(each.key(), i);
        }
        Some(store)
    }

    pub fn from_file(file_name: &str) -> Option<Store<T>> {
        if let Ok(f) = File::open(file_name){
            let m: Option<Vec<T>> = if let Ok(data) = ron::de::from_reader(f) { Some(data) } else {None};
            if m.is_none() {
                return None;
            }
            let mut store: Store<T> = Store{
                matrix: m.unwrap(),
                index_map: HashMap::new()
            };
            for (i, each) in store.matrix.iter().enumerate() {
                store.index_map.insert(each.key(), i);
            }
            return Some(store);
        }
        return None;
    }

    pub fn from_str(s: &str) -> Result<Store<T>, std::io::Error> {
        match ron::de::from_str(s) {
            Ok(m) => {
                let mut store: Store<T> = Store {
                    matrix: m,
                    index_map: HashMap::new()
                };
                for (i, each) in store.matrix.iter().enumerate() {
                    store.index_map.insert(each.key(), i);
                }
                Ok(store)
            }
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        }
    }

    fn check_for_rebuild(&self) -> bool {
        self.index_map.len() != self.matrix.len() && self.matrix.len() != 0
    }

    pub fn push(&mut self, val: T) {
        self.index_map.insert(val.key(), self.matrix.len());
        self.matrix.push(val);
    }

    pub fn get(&self, k: T::Key) -> Result<Option<&T>, engine::Error> {
        if self.check_for_rebuild() {
            return Err(engine::Error::RebuildRequired(None))
        }
        match self.index_map.get(&k) {
            Some(index) => Ok(self.matrix.get(*index)),
            None => Ok(None)
        }
    }

    pub fn get_mut(&mut self, k: &T::Key) -> Option<&mut T> {
        match self.index_map.get(k) {
            Some(index) => self.matrix.get_mut(*index),
            None => None
        }
    }

    pub fn to_file(&self, file_name: &str) -> Result<(), std::io::Error> {
        match File::open(file_name){
            Ok(mut f) => {
                let mut s = String::new();
                match ron::ser::to_writer_pretty(&mut s, &self.matrix, ron::ser::PrettyConfig::default()) {
                    Ok(_) => {}
                    Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Ron Failed to Write data"))
                }
                return f.write_all(s.as_bytes());
            }
            Err(e) => return Err(e)
        }
    }
}
