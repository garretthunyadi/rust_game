// use crate::l;
use crate::l;
use crate::locations::{Location, Locations};
use crate::maybe;
// use crate::s;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct EntityLocation<'a> {
    pub locations: &'a Locations,
    rng: ThreadRng,
    pub curr: Option<Location>,
    num_moves: u32,
}

impl<'a> EntityLocation<'a> {
    pub fn new(locations: &'a Locations, curr: Option<Location>) -> EntityLocation<'a> {
        EntityLocation {
            locations,
            rng: rand::thread_rng(),
            curr,
            num_moves: 0,
        }
    }
    pub fn new_rand(locations: &'a Locations) -> EntityLocation<'a> {
        let mut rng = rand::thread_rng();

        let curr = Some(locations.rand(&mut rng));

        EntityLocation {
            locations,
            rng,
            curr,
            num_moves: 0,
        }
    }
    pub fn rand_next_location_from(&self, rng: &mut ThreadRng) -> Option<Location> {
        let options = self.locations.destinations_from(
            &(self
                .curr
                .clone()
                .or_else(|| Some(self.locations.default()))
                .unwrap()),
        );
        match options.choose(rng) {
            Some(loc) => Some(loc.clone()),
            None => None,
        }
    }
    // pub fn can_move_to(&self, loc: &Location) -> bool {
    //     if let Some(c) = self.curr {
    //         if self.locations.destinations_from(&self.curr).contains(loc)

    //     }
    // }
    pub fn move_to(&mut self, loc: &Location) {
        self.curr = Some(loc.clone());
    }
}

impl<'a> Iterator for EntityLocation<'a> {
    // we will be counting with usize
    type Item = Location;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        let options = self
            .locations
            .destinations_from(&self.curr.clone().unwrap_or_else(|| l!["storage"]));
        match options.choose(&mut self.rng) {
            Some(loc) => {
                self.curr = Some(loc.clone());
                self.num_moves += 1;
                maybe::maybe(&mut self.rng, || println!("visiting: {:?}", loc));
            }
            None => {
                self.curr = None;
            }
        };
        self.curr.clone()
    }
}
