pub mod creature_state;
pub mod creatures;
pub mod entity_locations;
pub mod game_state;
pub mod locations;
pub mod log;
pub mod macros;
pub mod maybe;
pub mod rooms;

use rand::prelude::ThreadRng;

pub trait Map<L> {
    fn starting_location() -> L;
    fn rand_location(rng: &mut rand::prelude::ThreadRng) -> L;
    fn connections(&self) -> Vec<L>;
    fn rand_connection(&self, rng: &mut rand::prelude::ThreadRng) -> Option<L>;
}

// TODO: I'm not specifying anything about the (L)ocation implementing the map trait.
// If I try, I get that the trait cannot be made into an object.
pub struct Creature<L>
where
    L: Map<L>,
    L: Copy,
{
    name: String,
    loc: Option<L>,
    rng: ThreadRng,
}

impl<L> Creature<L>
where
    L: Map<L>,
    L: Copy,
{
    pub fn new(name: String) -> Creature<L> {
        Creature {
            name,
            loc: None,
            rng: rand::thread_rng(),
        }
    }
    pub fn place(&mut self, location: L) {
        self.loc = Some(location);
    }
    /// Move to a place that is one move away from the current location
    pub fn move_randomly(&mut self, rng: &mut rand::prelude::ThreadRng) -> Option<L> {
        if let Some(l) = self.loc {
            self.loc = l.rand_connection(rng);
        }
        self.loc
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn location(&self) -> Option<L> {
        self.loc
    }
}
impl<L> Iterator for Creature<L>
where
    L: Map<L>,
    L: Copy,
{
    type Item = L;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(l) = self.loc {
            self.loc = l.rand_connection(&mut self.rng);
        }
        self.loc
    }
}
