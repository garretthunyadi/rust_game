use crate::s;
use std::collections::HashMap;
// use std::marker::PhantomData;
use crate::rand::prelude::IteratorRandom;
// use rand::prelude::ThreadRng;
// use rand::seq::SliceRandom;

// pub type Location = String;
#[derive(Hash, Eq, Debug, PartialEq, Clone)]
pub struct Location(String);

#[derive(Debug)]
pub struct Locations {
    map: HashMap<Location, Vec<Location>>,
}

macro_rules! l {
    ($name:expr) => {
        Location(String::from($name))
    };
}

impl Locations {
    pub fn new() -> Locations {
        let mut map = HashMap::new();
        map.insert(l!("driveway"), vec![l!("front_door"), l!("garage")]);
        map.insert(l!("front_door"), vec![l!("driveway"), l!("garage")]);
        map.insert(
            l!("garage"),
            vec![l!("driveway"), l!("front_door"), l!("side_door")],
        );
        map.insert(l!("side_door"), vec![]);
        Locations { map }
    }

    pub fn starting(&self) -> Location {
        Location::from(*self.map.keys().nth(0).unwrap())
    }

    pub fn rand(&self, rng: &mut rand::prelude::ThreadRng) -> Location {
        Location::from(*self.map.keys().choose(rng).unwrap())
    }

    pub fn destinations_from(&self, id: &Location) -> Vec<Location> {
        match self.map.get(id) {
            Some(v) => v.to_vec(),
            None => vec![],
        }
    }
}
