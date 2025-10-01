use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SparseDataObject<T> {
    element: T,
    key: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SparseSet<T> {
    dense: Vec<SparseDataObject<T>>,
    sparse: Vec<usize>,
}

impl<T> SparseSet<T> {
    pub fn new(size: usize) -> Self {
        Self {
            dense: Vec::new(),
            sparse: vec![usize::MAX; size],
        }
    }

    pub fn clear(&mut self) {
        self.dense.clear();
        self.sparse.fill(usize::MAX);
    }

    pub fn insert(&mut self, key: usize, data: T) {
        if self.sparse[key] != usize::MAX {
            let index = self.sparse[key];
            self.dense[index].element = data;
        } else {
            let index = self.dense.len();
            self.dense.push(SparseDataObject {
                element: data,
                key: key,
            });
            self.sparse[key] = index;
        }
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        let index = self.sparse.get(id)?;
        if *index == usize::MAX {
            None
        } else {
            Some(&self.dense[*index].element)
        }
    }

    pub fn is_filled(&self, id: usize) -> bool {
        self.sparse[id] != usize::MAX
    }

    // id = 0
    pub fn remove(&mut self, id: usize) -> Option<T> {
        let data_index = self.sparse.get_mut(id)?; // 1
        if *data_index == usize::MAX {
            return None;
        }

        let index = *data_index; // 1
        let rm = self.dense.swap_remove(index); // rm = { e: 'C', k: 0 } 
        // self.dense[index] = {e: 'F', k: 3
        self.sparse[self.dense[index].key] = index;
        self.sparse[id] = usize::MAX;
        Some(rm.element)
    }
}
