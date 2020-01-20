// extern crate rand;

// use super::creatures::Creature;
// use super::entity_locations::EntityLocation;
// use super::locations::Locations;
// use rand::prelude::ThreadRng;

// pub struct CreatureState<'a, L>
// where
//     L: Clone,
//     L: Eq,
//     L: PartialEq,
//     L: std::fmt::Debug,
//     L: std::hash::Hash,
//     L: crate::engine::locations::Map<L>,
// {
//     pub creature: Creature,
//     pub location: EntityLocation<'a, L>,
// }

// impl<'a, L> CreatureState<'a, L>
// where
//     L: Clone,
//     L: Eq,
//     L: PartialEq,
//     L: std::fmt::Debug,
//     L: std::hash::Hash,
//     L: crate::engine::locations::Map<L>,
// {
//     pub fn new(locations: &'a Locations<L>) -> CreatureState<'a, L> {
//         CreatureState {
//             creature: Creature::rand(),
//             location: EntityLocation::new(locations, None),
//         }
//     }

//     pub fn next(&mut self, rng: &mut ThreadRng) {
//         self.location.move_to(
//             &self
//                 .location
//                 .rand_next_location_from(rng)
//                 .unwrap_or_else(|| self.location.locations.default()),
//         );
//     }
// }
