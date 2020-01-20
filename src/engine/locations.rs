// // use crate::s;
// // use std::collections::HashMap;
// // use std::fmt;
// // use std::marker::PhantomData;
// // use crate::rand::prelude::IteratorRandom;
// // use rand::prelude::ThreadRng;
// // use rand::seq::SliceRandom;

// // pub type Location = String;
// // impl ToString for Location {
// //     fn to_string(&self) -> String {
// //         self
// //     }
// // }

// // impl fmt::Display for Location {
// //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// //         write!(f, "{:?}", self)
// //         // or, alternatively:
// //         // fmt::Debug::fmt(self, f)
// //     }
// // }

// pub trait Map<L> {
//     fn default(&self) -> L;
//     fn rand(&self, rng: &mut rand::prelude::ThreadRng) -> L;
//     fn destinations_from(&self, id: &L) -> Vec<L>;
// }

// #[derive(Debug)]
// pub struct Locations<L>
// where
//     L: Clone,
//     L: Eq,
//     L: PartialEq,
//     L: std::fmt::Debug,
//     L: std::hash::Hash,
//     L: Map<L>, {
//     // map: HashMap<L, Vec<L>>,
// // map: Box<dyn Map<L>>,
// }

// impl<L> Locations<L>
// where
//     L: Clone,
//     L: Eq,
//     L: PartialEq,
//     L: std::fmt::Debug,
//     L: std::hash::Hash,
//     L: Map<L>,
// {
//     // pub fn new(map: HashMap<L, Vec<L>>) -> Locations<L> {
//     pub fn new(map: Box<dyn Map<L>>) -> Locations<L> {
//         Locations<L> {}
//     }

//     pub fn default(&self) -> L {
//         // (*self.map.keys().nth(0).unwrap()).clone()
//         self.map.default()
//     }

//     pub fn rand(&self, rng: &mut rand::prelude::ThreadRng) -> L {
//         // (*self.map.keys().choose(rng).unwrap()).clone()
//         self.map.rand(rng)
//     }

//     pub fn destinations_from(&self, from: &L) -> Vec<L> {
//         // match self.map.get(id) {
//         //     Some(v) => v.to_vec(),
//         //     None => vec![],
//         // }
//         self.map.destinations_from(from)
//     }
// }
