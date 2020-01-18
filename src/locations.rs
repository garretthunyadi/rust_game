use crate::s;
use std::collections::HashMap;
// use std::marker::PhantomData;

pub type Location = String;

pub struct Locations {
    map: HashMap<Location, Vec<Location>>,
}

impl Locations {
    pub fn new() -> Locations {
        let mut map = HashMap::new();
        map.insert(s!("driveway"), vec![s!("front_door"), s!("garage")]);
        map.insert(s!("front_door"), vec![s!("driveway"), s!("garage")]);
        map.insert(
            s!("garage"),
            vec![s!("driveway"), s!("front_door"), s!("side_door")],
        );
        map.insert(s!("side_door"), vec![]);
        Locations { map }
    }

    pub fn stating(&self) -> Location {
        self.map.keys().nth(0).unwrap().clone()
    }

    pub fn destinations(&self, id: &str) -> Vec<Location> {
        match self.map.get(id) {
            Some(v) => v.clone(),
            None => vec![],
        }
    }
}
