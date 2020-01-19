#![allow(dead_code)]

///
/// Game: Just to work through design decisions and refactorings
///
/// TODO:
/// [] Remove HashMap map ifo a fn with a exhaustive match (Make error states unrepresentable)
///
extern crate rand;

mod engine;

use engine::creature_state::CreatureState;
use engine::entity_locations::EntityLocation;
use engine::locations::Locations;
use engine::log::Log;
// use rand::prelude::ThreadRng;

mod game1 {
    use std::collections::HashMap;

    #[derive(Hash, Eq, Debug, PartialEq, Clone)]
    pub enum Location {
        Yard,
        FrontDoor,
        Garage,
        Driveway,
        SideDoor,
        Nowhere,
    }
    impl Location {
        pub fn map() -> HashMap<Location, Vec<Location>> {
            use Location::*;
            let mut map = HashMap::new();
            map.insert(Yard, vec![FrontDoor, Garage]);
            map.insert(FrontDoor, vec![Driveway, Garage]);
            map.insert(Garage, vec![Driveway, FrontDoor, SideDoor]);
            map.insert(SideDoor, vec![]);
            map
        }
    }
}

mod game2 {
    use std::collections::HashMap;

    #[derive(Hash, Eq, Debug, PartialEq, Clone)]
    pub enum Location {
        MallEntrance,
        ParkingLot,
        Concourse,
        Gap,
        FoodCourt,
        Macys,
        Exit,
    }
    impl Location {
        pub fn map() -> HashMap<Location, Vec<Location>> {
            use Location::*;
            let mut map = HashMap::new();
            map.insert(MallEntrance, vec![ParkingLot, Concourse]);
            map.insert(ParkingLot, vec![MallEntrance]);
            map.insert(Concourse, vec![Gap, FoodCourt, Macys, Exit]);
            map.insert(Gap, vec![Concourse]);
            map.insert(FoodCourt, vec![Concourse]);
            map.insert(Macys, vec![Concourse]);
            map.insert(Exit, vec![]);
            map
        }
    }
}

fn main() {
    game1();
    game2()
}
fn game1() {
    let mut rng = rand::thread_rng();
    let log = Log::new();
    let map = Locations::new(game1::Location::map());
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
fn game2() {
    let mut rng = rand::thread_rng();
    let log = Log::new();
    let map = Locations::new(game2::Location::map());
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
