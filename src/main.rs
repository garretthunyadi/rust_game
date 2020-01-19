#![allow(dead_code)]
extern crate rand;

mod engine;

use engine::creature_state::CreatureState;
use engine::entity_locations::EntityLocation;
use engine::locations::Locations;
use engine::log::Log;
// use rand::prelude::ThreadRng;

fn main() {
    let mut rng = rand::thread_rng();
    let log = Log::new();
    let map = Locations::new();
    let mut pos = EntityLocation::new(&map, Some(map.default()));
    let mut cs = CreatureState::new(&map);

    cs.next(&mut rng);
    puto!(cs.location);
    cs.next(&mut rng);
    puto!(cs.location);
    cs.next(&mut rng);
    puto!(cs.location);

    while pos.next().is_some() {
        let p = pos.curr.clone().unwrap();
        puto!(p);
        log.log(&format!("{:?}", p));
    }

    puts!("===FIN===");
    puts!(log);
}
