#![allow(dead_code)]
extern crate rand;

mod creatures;
mod entity_locations;
mod game_state;
mod locations;
mod macros;
mod maybe;
mod rooms;

use entity_locations::EntityLocation;
use locations::Locations;

fn main() {
    let map = Locations::new();
    let mut pos = EntityLocation::new(&map, Some(map.stating()));
    while pos.next().is_some() {
        puts!(pos.curr.clone().unwrap());
    }
}
