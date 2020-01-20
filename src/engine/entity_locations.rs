// // use crate::l;
// use crate::engine::locations::Locations;
// use crate::engine::maybe;
// // use crate::s;
// use rand::prelude::ThreadRng;
// use rand::seq::SliceRandom;

// #[derive(Debug)]
// pub struct EntityLocation<'a, L>
// where
//     L: Clone,
//     L: Eq,
//     L: PartialEq,
//     L: std::fmt::Debug,
//     L: std::hash::Hash,
//     L: crate::engine::locations::Map<L>,
// {
//     pub locations: &'a Locations<L>,
//     rng: ThreadRng,
//     pub curr: Option<L>,
//     num_moves: u32,
// }

// impl<'a, L> EntityLocation<'a, L>
// where
//     L: Clone,
//     L: Eq,
//     L: PartialEq,
//     L: std::fmt::Debug,
//     L: std::hash::Hash,
//     L: crate::engine::locations::Map<L>,
// {
//     pub fn new(locations: &'a Locations<L>, curr: Option<L>) -> EntityLocation<'a, L> {
//         EntityLocation {
//             locations,
//             rng: rand::thread_rng(),
//             curr,
//             num_moves: 0,
//         }
//     }
//     pub fn new_rand(locations: &'a Locations<L>) -> EntityLocation<'a, L> {
//         let mut rng = rand::thread_rng();

//         let curr = Some(locations.rand(&mut rng));

//         EntityLocation {
//             locations,
//             rng,
//             curr,
//             num_moves: 0,
//         }
//     }
//     pub fn rand_next_location_from(&self, rng: &mut ThreadRng) -> Option<L> {
//         let options = self.locations.destinations_from(
//             &(self
//                 .curr
//                 .clone()
//                 .or_else(|| Some(self.locations.default()))
//                 .unwrap()),
//         );
//         match options.choose(rng) {
//             Some(loc) => Some(loc.clone()),
//             None => None,
//         }
//     }
//     // pub fn can_move_to(&self, loc: &Location) -> bool {
//     //     if let Some(c) = self.curr {
//     //         if self.locations.destinations_from(&self.curr).contains(loc)

//     //     }
//     // }
//     pub fn move_to(&mut self, loc: &L) {
//         self.curr = Some(loc.clone());
//     }
// }

// impl<'a, L> Iterator for EntityLocation<'a, L>
// where
//     L: Clone,
//     L: Eq,
//     L: PartialEq,
//     L: std::fmt::Debug,
//     L: std::hash::Hash,
//     L: crate::engine::locations::Map<L>,
// {
//     type Item = L;

//     fn next(&mut self) -> Option<Self::Item> {
//         let options = self
//             .locations
//             // .destinations_from(&self.curr.clone().unwrap_or_else(|| Location::Nowhere)); //TODO
//             .destinations_from(&self.curr.clone().unwrap());
//         match options.choose(&mut self.rng) {
//             Some(loc) => {
//                 self.curr = Some(loc.clone());
//                 self.num_moves += 1;
//                 maybe::maybe(&mut self.rng, || println!("visiting: {:?}", loc));
//             }
//             None => {
//                 self.curr = None;
//             }
//         };
//         self.curr.clone()
//     }
// }
