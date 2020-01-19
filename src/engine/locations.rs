// use crate::s;
use std::collections::HashMap;
// use std::fmt;
// use std::marker::PhantomData;
use crate::rand::prelude::IteratorRandom;
// use rand::prelude::ThreadRng;
// use rand::seq::SliceRandom;

// pub type Location = String;
// impl ToString for Location {
//     fn to_string(&self) -> String {
//         self
//     }
// }

// impl fmt::Display for Location {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//     }
// }

#[derive(Debug)]
pub struct Locations<L>
where
    L: Clone,
    L: Eq,
    L: PartialEq,
    L: std::fmt::Debug,
    L: std::hash::Hash,
{
    map: HashMap<L, Vec<L>>,
}

impl<L> Locations<L>
where
    L: Clone,
    L: Eq,
    L: PartialEq,
    L: std::fmt::Debug,
    L: std::hash::Hash,
{
    pub fn new(map: HashMap<L, Vec<L>>) -> Locations<L> {
        Locations { map }
    }

    pub fn default(&self) -> L {
        (*self.map.keys().nth(0).unwrap()).clone()
    }

    pub fn rand(&self, rng: &mut rand::prelude::ThreadRng) -> L {
        (*self.map.keys().choose(rng).unwrap()).clone()
    }

    pub fn destinations_from(&self, id: &L) -> Vec<L> {
        match self.map.get(id) {
            Some(v) => v.to_vec(),
            None => vec![],
        }
    }
}
