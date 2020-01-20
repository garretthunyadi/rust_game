use crate::engine::Map;
// use crate::s;
use rand::prelude::ThreadRng;

#[derive(Debug)]
pub enum Disposition {
    Friendly,
    Neutral,
    Hostile,
}

pub trait Disposed {
    fn disposition(&self) -> Disposition;
}

#[derive(Debug, Clone, Copy)]
pub struct HP(pub u32);

pub trait DefaultHP {
    fn default_hp(&self) -> HP;
    fn hp(&self) -> HP;
    fn increase_hp(&mut self, by: HP) -> HP;
    fn decrease_hp(&mut self, by: HP) -> HP;
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
    pub fn default_hp(&self) -> HP {
        use Species::*;
        match self {
            Human => HP(100),
            Cow => HP(80),
            Pig => HP(15),
            Snake => HP(5),
            Orc => HP(80),
        }
    }
    pub fn default_disposition(&self) -> Disposition {
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
    pub species: Species,
    pub hp: HP,
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

pub fn interact<C: Disposed + DefaultHP>(
    c1: &mut C,
    c2: &mut C,
    rng: &mut rand::prelude::ThreadRng,
) {
    match (c1.disposition(), c2.disposition()) {
        (Disposition::Hostile, Disposition::Hostile) => fight(c1, c2, rng),
        (_, Disposition::Hostile) => {
            if rand::random() {
                fight(c1, c2, rng)
            }
        }
        (Disposition::Hostile, _) => {
            if rand::random() {
                fight(c1, c2, rng)
            }
        }
        _ => {}
    }
}
use crate::rand::Rng;
pub fn fight<T: DefaultHP>(c1: &mut T, c2: &mut T, rng: &mut rand::prelude::ThreadRng) {
    // TODO: not all fights are to the death,
    // TODO: nor are they this simple
    // unimplemented!();
    while c1.hp().0 > 0 && c2.hp().0 > 0 {
        if rng.gen() {
            c1.decrease_hp(HP(1));
        } else {
            c2.decrease_hp(HP(1));
        }
    }
}
