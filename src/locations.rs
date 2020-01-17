use crate::s;
use std::collections::HashMap;
use std::marker::PhantomData;

type Location = String;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Locations {
    map: HashMap::new(),
}

impl Locations {
    // pub fn next(&self, &Location) -> Location {
    //     self.map.
    // }

    pub fn destinations(&self, id: &Location) -> Vec<Location> {
        self.map.get(id)
    }
}
