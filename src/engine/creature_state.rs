extern crate rand;

use super::creatures::Creature;
use super::entity_locations::EntityLocation;
use super::locations::Locations;
use rand::prelude::ThreadRng;

pub struct CreatureState<'a> {
    pub creature: Creature,
    pub location: EntityLocation<'a>,
}

impl<'a> CreatureState<'a> {
    pub fn new(locations: &'a Locations) -> CreatureState<'a> {
        CreatureState {
            creature: Creature::rand(),
            location: EntityLocation::new(locations, None),
        }
    }

    pub fn next(&mut self, rng: &mut ThreadRng) {
        self.location.move_to(
            &self
                .location
                .rand_next_location_from(rng)
                .unwrap_or_else(|| self.location.locations.default()),
        );
    }
}
