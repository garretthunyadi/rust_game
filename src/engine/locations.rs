// use crate::s;
use std::collections::HashMap;
// use std::fmt;
// use std::marker::PhantomData;
use crate::rand::prelude::IteratorRandom;
// use rand::prelude::ThreadRng;
// use rand::seq::SliceRandom;

// pub type Location = String;
#[derive(Hash, Eq, Debug, PartialEq, Clone)]
pub enum Location {
    Yard,
    FrontDoor,
    Garage,
    Driveway,
    SideDoor,
    Nowhere,
}

// impl ToString for Location {
//     fn to_string(&self) -> String {
//         self
//     }
// }

// impl fmt::Display for Location {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//     }
// }

#[derive(Debug)]
pub struct Locations {
    map: HashMap<Location, Vec<Location>>,
}

impl Locations {
    pub fn new() -> Locations {
        use Location::*;
        let mut map = HashMap::new();
        map.insert(Yard, vec![FrontDoor, Garage]);
        map.insert(FrontDoor, vec![Driveway, Garage]);
        map.insert(Garage, vec![Driveway, FrontDoor, SideDoor]);
        map.insert(SideDoor, vec![]);
        Locations { map }
    }

    pub fn default(&self) -> Location {
        self.map.keys().nth(0).unwrap().clone()
    }

    pub fn rand(&self, rng: &mut rand::prelude::ThreadRng) -> Location {
        self.map.keys().choose(rng).unwrap().clone()
    }

    pub fn destinations_from(&self, id: &Location) -> Vec<Location> {
        match self.map.get(id) {
            Some(v) => v.to_vec(),
            None => vec![],
        }
    }
}
