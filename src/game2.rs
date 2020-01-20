use super::puts;
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

pub fn play() {
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

// use crate::engine::Map;
// use std::collections::HashMap;

// #[derive(Hash, Eq, Debug, PartialEq, Clone)]
// pub enum Location {
//     MallEntrance,
//     ParkingLot,
//     Concourse,
//     Gap,
//     FoodCourt,
//     Macys,
//     Exit,
// }
// impl Location {
//     pub fn map() -> HashMap<Location, Vec<Location>> {
//         use Location::*;
//         let mut map = HashMap::new();
//         map.insert(MallEntrance, vec![ParkingLot, Concourse]);
//         map.insert(ParkingLot, vec![MallEntrance]);
//         map.insert(Concourse, vec![Gap, FoodCourt, Macys, Exit]);
//         map.insert(Gap, vec![Concourse]);
//         map.insert(FoodCourt, vec![Concourse]);
//         map.insert(Macys, vec![Concourse]);
//         map.insert(Exit, vec![]);
//         map
//     }
// }
// impl Map<Location> for Location {
//     fn starting_location() -> Location {
//         Location::MallEntrance
//     }
//     fn rand_location(_rng: &mut rand::prelude::ThreadRng) -> Location {
//         Location::MallEntrance
//     }
//     fn connections(&self) -> Vec<Location> {
//         vec![]
//     }
//     fn rand_connection(&self, _rng: &mut rand::prelude::ThreadRng) -> Option<Location> {
//         None // TODO
//     }
// }

// impl Default for Location {
//     fn default() -> Self {
//         Location::Exit // Terminal Location
//     }
// }
