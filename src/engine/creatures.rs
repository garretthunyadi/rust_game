use crate::s;

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
}

#[derive(Debug)]
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
