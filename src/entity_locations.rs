use crate::locations::{Location, Locations};
use crate::maybe;
use crate::s;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;

pub struct EntityLocation<'a> {
    locations: &'a Locations,
    rng: ThreadRng,
    pub curr: Option<Location>,
}

impl<'a> EntityLocation<'a> {
    pub fn new(locations: &'a Locations, curr: Option<Location>) -> EntityLocation<'a> {
        EntityLocation {
            locations,
            rng: rand::thread_rng(),
            curr,
        }
    }
}

impl<'a> Iterator for EntityLocation<'a> {
    // we will be counting with usize
    type Item = Location;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        let options = self
            .locations
            .destinations(&self.curr.clone().unwrap_or_else(|| s!["storage"]));
        match options.choose(&mut self.rng) {
            Some(loc) => {
                self.curr = Some(s![loc]);
                maybe::maybe(&mut self.rng, || println!("visiting: {}", loc));
            }
            None => {
                self.curr = None;
            }
        };
        self.curr.clone()
    }
}
