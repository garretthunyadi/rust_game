pub mod creature_state;
pub mod creatures;
pub mod entity_locations;
pub mod game_state;
pub mod locations;
pub mod log;
pub mod macros;
pub mod maybe;
pub mod rooms;

pub trait Map<L> {
    fn starting_location() -> L;
    fn rand_location(rng: &mut rand::prelude::ThreadRng) -> L;
    fn connections(&self) -> Vec<L>;
    fn rand_connection(&self, rng: &mut rand::prelude::ThreadRng) -> Option<L>;
}

// impl Iterator for Map<L> {
//     type Item = L;

//     fn next(&mut self) -> Option<Self::Item> {
//         None
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
