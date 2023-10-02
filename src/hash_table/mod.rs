use std::borrow::Borrow;
use std::hash::Hash;

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
        todo!()
    }

    pub fn insert(&mut self, key: Key, val: Val) -> Option<Val> {
        todo!()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Val> 
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        todo!()
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
}