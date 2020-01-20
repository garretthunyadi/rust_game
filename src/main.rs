#![allow(dead_code)]

///
/// Game: Just to work through design decisions and refactorings
///
/// TODO:
/// [x] Remove HashMap map ifo a fn with a exhaustive match (Make error states unrepresentable)
/// [] Create Creatures and make them interact
///
extern crate rand;
mod engine;
use engine::Map;

mod game1 {
    use super::engine::Map;

    #[derive(Hash, Eq, Debug, PartialEq, Clone, Copy)]
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
                Some(&l) => Some(l),
                None => None,
            }
        }
    }
    impl Default for Location {
        fn default() -> Self {
            Location::Nowhere
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
        fn rand_location(_rng: &mut rand::prelude::ThreadRng) -> Location {
            Location::MallEntrance
        }
        fn connections(&self) -> Vec<Location> {
            vec![]
        }
        fn rand_connection(&self, _rng: &mut rand::prelude::ThreadRng) -> Option<Location> {
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
    use crate::engine::creatures;
    use crate::engine::creatures::{Creature, Species};
    use crate::game1::*;

    let mut rng = rand::thread_rng();
    // let log = Log::new();

    let mut curr = Some(Location::starting_location());
    while let Some(loc) = curr {
        println!("{:?}", loc);
        // log.log(&format!("{:?}", loc));
        curr = loc.rand_connection(&mut rng);
    }

    let mut c1 = Creature::new(s!("Bob"), Species::Human);
    c1.place(Location::starting_location());

    while let Some(loc) = c1.location() {
        println!("{} is at {:?}", c1.name(), loc);
        c1.move_randomly(&mut rng);
    }

    let mut c2 = Creature::new(s!("Ugg"), Species::Orc);
    c2.place(Location::starting_location());

    while let Some(loc) = c2.location() {
        println!("{} is at {:?}", c2.name(), loc);
        c2.move_randomly(&mut rng);
    }

    println!("BEFORE INTERACTION: \n  {:?}\n  {:?}", c1, c2);
    creatures::interact(&mut c1, &mut c2);
    println!("AFTER INTERACTION: \n  {:?}\n  {:?}", c1, c2);

    puts!("===FIN 1===");
    // puts!(log);
}

fn play_game2() {
    use crate::game2::*;

    let mut rng = rand::thread_rng();
    // let log = Log::new();
    let mut curr = Some(Location::starting_location());
    while let Some(loc) = curr {
        println!("{:?}", loc);
        // log.log(&format!("{:?}", loc));
        curr = loc.rand_connection(&mut rng);
    }

    puts!("===FIN 2 ===");
    // puts!(log);
}
