use std::borrow::Borrow;
use std::hash::Hash;

pub struct HashMap<Key, Val> {
    // todo
}

impl <Key: Eq + Hash, Val> HashMap<Key, Val> {
    pub fn new() -> Self {
        todo!()
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