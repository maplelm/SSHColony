use crate::engine::Error;

use super::super::traits::Storeable;
use super::super::super::engine;
use std::{collections::HashMap, fs::read_dir};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound = "T: Storeable")]
pub struct Store<T: Storeable>
where T::Key: Eq + std::hash::Hash, {
    matrix: Vec<T>,
    #[serde(default="HashMap::new")]
    index_map: HashMap<T::Key, usize>
}


impl<T: Storeable> Default for Store<T>
where T::Key: Eq + std::hash::Hash, {
    fn default() -> Self {
        Self {
            matrix: vec![],
            index_map: HashMap::new()
        }
    }
}


impl<T: Storeable> Store<T> 
where T::Key: Eq + std::hash::Hash {

    ////////////////////////
    //  Public Functions  //
    ////////////////////////
    pub fn from_dir(dir: &str) -> Result<Self, std::io::Error> {
        let dir = read_dir(dir);
        if let Err(dir) = dir {
            // Log that there is a problem
            print!("\rFailed to read dir: {}\r\n", dir);
            return Err(dir);
        }
        let mut dir = dir.unwrap();
        let mut store = Store::<T>::default();
        while let Some(Ok(entry)) = dir.next() {
            #[cfg(debug_assertions)]
            println!("\rprocessing file {}", entry.path().to_str().unwrap());
            if let Ok(t) = entry.file_type() && t.is_file() {
                let pathbuf = entry.path();
                let path = pathbuf.to_str();
                let namebuf = entry.file_name();
                let name = namebuf.to_str();
                if path.is_none() || name.is_none() {
                    // Log that there is a problem
                    continue;
                }

                let file = std::fs::File::open(path.unwrap().to_string());
                if file.is_err() {
                    // Log that there is a problem
                    #[cfg(debug_assertions)]
                    println!("\rFailed to open file: {} | {}",path.unwrap().to_string()+name.unwrap(), file.unwrap_err());
                    continue;
                }
                let mut m: Option<Vec<T>> = None;
                match ron::de::from_reader(file.as_ref().unwrap()) {
                    Ok(r) => m = Some(r),
                    Err(e) => {
                        // log that there is a problem
                        println!("\ron::de::from_reader failed for {:?} \n\r{}", file.unwrap(), e);
                    }
                }
                if m.is_none(){
                    // log that there is a problem
                    continue;
                }
                let max = m.as_ref().unwrap().len();
                for _ in 0..max {
                    store.matrix.push(m.as_mut().unwrap().swap_remove(0));
                }
            }
        }
        store.rebuild();
        if store.matrix.len() == 0 {
            // log that there is a problem
            println!("\rStore Matrix is empty");
        }
        return Ok(store);
    }

    pub fn from_file(path: &str) -> Option<Self> {
        todo!()
    }

    pub fn from_str(s: &str) -> Option<Self> {
        todo!()
    }

    pub fn push(&mut self, val: T) {
        todo!()
    }

    pub fn get(&self, k: T::Key) -> Result<Option<&T>, engine::Error> {
        Ok(None)
    }

    pub fn get_mut(&mut self, k: T::Key) -> Result<Option<&mut T>, engine::Error> {
        Ok(None)
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        let mut s = Self {
            matrix: v,
            index_map: HashMap::new()
        };
        s.rebuild();
        return s;
    }

    pub fn rebuild(&mut self) {
        self.index_map.clear();
        for (index, each) in self.matrix.iter().enumerate() {
            self.index_map.insert(each.key(), index);
        }
    }

    //////////////////////////
    //  Private Functions  ///
    //////////////////////////
    
    fn need_rebuild(&self) -> bool {
        self.index_map.len() != self.matrix.len() && self.matrix.len() != 0
    }
}
