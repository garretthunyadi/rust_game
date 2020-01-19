#![allow(dead_code)]
extern crate rand;

mod creatures;
mod entity_locations;
mod game_state;
mod locations;
mod log;
mod macros;
mod maybe;
mod rooms;

use creatures::Creature;
use entity_locations::EntityLocation;
use locations::Locations;
use log::Log;
use rand::prelude::ThreadRng;

fn main() {
    let mut rng = rand::thread_rng();
    let log = Log::new();
    let map = Locations::new();
    let mut pos = EntityLocation::new(&map, Some(map.starting()));
    let mut cs = CreatureState::new(&map);

    cs.next(&mut rng);
    puto!(cs.location);
    cs.next(&mut rng);
    puto!(cs.location);
    cs.next(&mut rng);
    puto!(cs.location);

    while pos.next().is_some() {
        let p = pos.curr.clone().unwrap();
        puts!(p);
        log.log(&p);
    }

    puts!("===FIN===");
    puts!(log);
}

struct CreatureState<'a> {
    creature: Creature,
    location: EntityLocation<'a>,
}

impl<'a> CreatureState<'a> {
    pub fn new(locations: &'a Locations) -> CreatureState<'a> {
        CreatureState {
            creature: Creature::rand(),
            location: EntityLocation::new(locations, None),
        }
    }

    pub fn next(&mut self, rng: &mut ThreadRng) {
        self.location
            .move_to(&self.location.rand_next_location_from(rng).unwrap());
    }
}
