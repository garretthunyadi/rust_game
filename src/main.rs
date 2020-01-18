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

use entity_locations::EntityLocation;
use locations::Locations;
use log::Log;

fn main() {
    let log = Log::new();
    let map = Locations::new();
    let mut pos = EntityLocation::new(&map, Some(map.stating()));
    while pos.next().is_some() {
        let p = pos.curr.clone().unwrap();
        puts!(p);
        log.log(&p);
    }

    puts!("===FIN===");
    puts!(log);
}
