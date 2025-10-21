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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SparseDataObject<T> {
    element: T,
    key: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseSet<T> {
    dense: Vec<T>,
    dense_keys: Vec<usize>,
    sparse: Vec<usize>,
    keys: HashMap<usize,()>
}

impl<T> SparseSet<T> {
    pub fn new(size: usize) -> Self {
        Self {
            dense: Vec::new(),
            dense_keys: Vec::new(),
            keys: HashMap::new(),
            sparse: vec![usize::MAX; size],
        }
    }

    pub fn clear(&mut self) {
        self.dense.clear();
        self.dense_keys.clear();
        self.keys.clear();
        self.sparse.fill(usize::MAX);
    }

    pub fn insert(&mut self, key: usize, data: T) {
        if self.sparse[key] != usize::MAX {
            // Key already exists
            let index = self.sparse[key];
            self.dense[index] = data;
        } else {
            // Adding new key
            let index = self.dense.len();
            self.dense.push(data);
            self.dense_keys.push(key);
            self.sparse[key] = index;
            self.keys.insert(key, ());
        }
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        let index = self.sparse.get(key)?;
        if *index == usize::MAX {
            None
        } else {
            self.dense.get_mut(*index)
        }
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        let index = self.sparse.get(key)?;
        if *index == usize::MAX {
            None
        } else {
            Some(&self.dense[*index])
        }
    }

    pub fn is_filled(&self, key: usize) -> bool {
        self.sparse[key] != usize::MAX
    }

    // id = 0
    pub fn remove(&mut self, key: usize) -> Option<T> {
        let data_index = self.sparse.get_mut(key)?; // 1
        if *data_index == usize::MAX {
            return None;
        }

        let index = *data_index; // 1
        let rm = self.dense.swap_remove(index); // rm = { e: 'C', k: 0 } 
        // self.dense[index] = {e: 'F', k: 3
        self.sparse[self.dense_keys[index]] = index;
        self.sparse[key] = usize::MAX;
        self.keys.remove(&key);
        Some(rm)
    }

    pub fn all_keys(&self) -> std::collections::hash_map::Iter<'_, usize, ()> {
        self.keys.iter()
        /*
        let mut result = Vec::<usize>::new();
        for each in self.sparse.iter() {
            if *each != usize::MAX {
                result.push(*each);
            }
        }
        result
        */
    }
}