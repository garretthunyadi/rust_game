#![allow(dead_code)]

///
/// Game: Just to work through design decisions and refactorings
///
/// TODO:
/// [] Remove HashMap map ifo a fn with a exhaustive match (Make error states unrepresentable)
///
extern crate rand;

mod engine;

// use engine::creature_state::CreatureState;
// use engine::entity_locations::EntityLocation;
// use engine::locations::Locations;
use engine::log::Log;
use engine::Map;

mod game1 {
    use super::engine::Map;

    #[derive(Hash, Eq, Debug, PartialEq, Clone)]
    pub enum Location {
        Yard,
        FrontDoor,
        Garage,
        Driveway,
        SideDoor,
        Couch,
        Nowhere,
    }

    impl Map<Location> for Location {
        fn starting_location() -> Location {
            Location::Yard
        }
        fn rand_location(rng: &mut rand::prelude::ThreadRng) -> Location {
            use crate::rand::Rng;
            use Location::*;
            match rng.gen_range(0, 6) {
                0 => Yard,
                1 => FrontDoor,
                2 => Garage,
                3 => Driveway,
                4 => SideDoor,
                5 => Couch,
                _ => Nowhere,
            }
        }
        fn connections(&self) -> Vec<Location> {
            use Location::*;
            match self {
                Yard => vec![FrontDoor, Garage],
                FrontDoor => vec![Driveway, Garage, Couch],
                Couch => vec![],
                Garage => vec![Driveway, FrontDoor, SideDoor],
                Driveway => vec![Garage, FrontDoor],
                SideDoor => vec![Garage, Driveway],
                Nowhere => vec![],
            }
        }
        fn rand_connection(&self, rng: &mut rand::prelude::ThreadRng) -> Option<Location> {
            use rand::seq::SliceRandom;
            match self.connections().choose(rng) {
                Some(l) => Some(l.clone()),
                None => None,
            }
        }
    }
    impl Default for Location {
        fn default() -> Self {
            Location::Nowhere
        }
    }
    impl Iterator for Location {
        type Item = Location;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }
}
mod game2 {
    use crate::engine::Map;
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
    impl Map<Location> for Location {
        fn starting_location() -> Location {
            Location::MallEntrance
        }
        fn rand_location(rng: &mut rand::prelude::ThreadRng) -> Location {
            Location::MallEntrance
        }
        fn connections(&self) -> Vec<Location> {
            vec![]
        }
        fn rand_connection(&self, rng: &mut rand::prelude::ThreadRng) -> Option<Location> {
            None // TODO
        }
    }

    impl Default for Location {
        fn default() -> Self {
            Location::Exit // Terminal Location
        }
    }
}

fn main() {
    puts!("===GAME 1 ===");

    play_game1();
    puts!("===GAME 2 ===");

    play_game2()
}
fn play_game1() {
    use crate::game1::*;

    let mut rng = rand::thread_rng();
    let log = Log::new();

    let mut curr = Some(Location::starting_location());
    while let Some(loc) = curr {
        println!("{:?}", loc);
        log.log(&format!("{:?}", loc));
        curr = loc.rand_connection(&mut rng);
    }
    // while curr.is_some()
    // curr = curr.rand_connection(&mut rng).unwrap_or_default();

    // let map = Locations::new(game1::Location);
    // let mut pos = EntityLocation::new(&map, Some(map.default()));
    // let mut cs = CreatureState::new(&map);

    // cs.next(&mut rng);
    // puto!(cs.location);
    // cs.next(&mut rng);
    // puto!(cs.location);
    // cs.next(&mut rng);
    // puto!(cs.location);

    // while pos.next().is_some() {
    //     let p = pos.curr.clone().unwrap();
    //     puto!(p);
    //     log.log(&format!("{:?}", p));
    // }

    puts!("===FIN 1===");
    puts!(log);
}
fn play_game2() {
    use crate::game2::*;

    let mut rng = rand::thread_rng();
    let log = Log::new();
    let mut curr = Some(Location::starting_location());
    while let Some(loc) = curr {
        println!("{:?}", loc);
        log.log(&format!("{:?}", loc));
        curr = loc.rand_connection(&mut rng);
    }

    // let map = Locations::new(game2::Location::map());
    // let mut pos = EntityLocation::new(&map, Some(map.default()));
    // let mut cs = CreatureState::new(&map);

    // cs.next(&mut rng);
    // puto!(cs.location);
    // cs.next(&mut rng);
    // puto!(cs.location);
    // cs.next(&mut rng);
    // puto!(cs.location);

    // while pos.next().is_some() {
    //     let p = pos.curr.clone().unwrap();
    //     puto!(p);
    //     log.log(&format!("{:?}", p));
    // }

    puts!("===FIN 2 ===");
    puts!(log);
}
