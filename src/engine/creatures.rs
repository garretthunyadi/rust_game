use crate::engine::Map;
// use crate::s;
use rand::prelude::ThreadRng;

#[derive(Debug)]
pub enum Disposition {
    Friendly,
    Neutral,
    Hostile,
}

#[derive(Debug)]
pub enum Species {
    Human,
    Cow,
    Pig,
    Snake,
    Orc,
}
impl Species {
    fn default_hp(&self) -> u32 {
        use Species::*;
        match self {
            Human => 100,
            Cow => 80,
            Pig => 15,
            Snake => 5,
            Orc => 80,
        }
    }
    fn default_disposition(&self) -> Disposition {
        use Disposition::*;
        use Species::*;
        match self {
            Human => Neutral,
            Cow => Neutral,
            Pig => Friendly,
            Snake => Hostile,
            Orc => Hostile,
        }
    }
}

// TODO: I'm not specifying anything about the (L)ocation implementing the map trait.
// If I try, I get that the trait cannot be made into an object.
#[derive(Debug)]
pub struct Creature<L>
where
    L: Map<L>,
    L: Copy,
{
    name: String,
    loc: Option<L>,
    rng: ThreadRng,
    disposition: Disposition,
    species: Species,
    hp: u32,
}

impl<L> Creature<L>
where
    L: Map<L>,
    L: Copy,
{
    pub fn new(name: String, species: Species) -> Creature<L> {
        Creature {
            name,
            loc: None,
            rng: rand::thread_rng(),
            disposition: species.default_disposition(),
            hp: species.default_hp(),
            species,
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

pub fn interact<L>(c1: &mut Creature<L>, c2: &mut Creature<L>)
where
    L: Map<L>,
    L: Copy,
{
}
