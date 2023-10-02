use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub enum Entry<Key, Val> {
    Vacant,
    Tombstone,
    Occupied { key: Key, val: Val }
}

pub struct HashMap<Key, Val> {
    xs: Vec<Entry<Key, Val>>,
    n_occupied: usize,
    n_vacant: usize
}

impl <Key: Eq + Hash, Val> HashMap<Key, Val> {
    pub fn new() -> Self {
        Self {
            xs: Vec::new(),
            n_occupied: 0,
            n_vacant: 0
        }
    }

    pub fn len(&self) -> usize {
        self.n_occupied
    }

    pub fn insert(&mut self, key: Key, val: Val) -> Option<Val> {
        todo!()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Val> 
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        if self.len() == 0 {
            return None
        }
        let mut idx = self.get_index(key);
        loop {
            match &self.xs[idx] {
                Entry::Vacant => {
                    break None;
                }
                Entry::Occupied { key: k, val } if k.borrow() == key => {
                    break Some(val);
                }
                _ => {
                    idx = (idx + 1) % self.xs.len();
                }
            }
        }
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Val>
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        todo!()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<Val>
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        todo!()
    }

    // Helper functions
    fn get_index<Q>(&self, key: &Q) -> usize
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.xs.len()
    }
}