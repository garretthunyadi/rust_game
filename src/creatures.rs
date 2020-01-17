use crate::s;

pub enum Disposition {
    Friendly,
    Neutral,
    Hostile,
}

pub enum Species {
    Human,
    Cow,
    Pig,
    Snake,
}

pub struct Creature {
    name: String,
    disposition: Disposition,
    species: Species,
    hp: u32,
}

impl Creature {
    pub fn rand() -> Creature {
        Creature {
            name: s!["Bob"],
            disposition: Disposition::Friendly,
            species: Species::Human,
            hp: 100,
        }
    }
}
