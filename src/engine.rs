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
