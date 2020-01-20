use super::engine::creatures::*;
use super::engine::Map;
use super::{puts, s};

#[derive(Hash, Eq, Debug, PartialEq, Clone, Copy)]
pub enum Location {
    MallEntrance,
    ParkingLot,
    Concourse,
    Gap,
    FoodCourt,
    Macys,
    Exit,
}

impl Map<Location> for Location {
    fn starting_location() -> Location {
        Location::MallEntrance
    }
    fn rand_location(rng: &mut rand::prelude::ThreadRng) -> Location {
        use crate::rand::Rng;
        use Location::*;
        match rng.gen_range(0, 6) {
            0 => MallEntrance,
            1 => ParkingLot,
            2 => Concourse,
            3 => Gap,
            4 => FoodCourt,
            5 => Macys,
            _ => Exit,
        }
    }
    fn connections(&self) -> Vec<Location> {
        use Location::*;
        match self {
            MallEntrance => vec![Concourse, ParkingLot],
            ParkingLot => vec![MallEntrance, Exit],
            Concourse => vec![MallEntrance, Gap, FoodCourt, Macys],
            Gap => vec![Concourse],
            FoodCourt => vec![Concourse],
            Macys => vec![Concourse],
            Exit => vec![],
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
        Location::MallEntrance
    }
}

pub fn play() {
    use super::*;
    use crate::engine::creatures;

    let mut rng = rand::thread_rng();
    // let log = Log::new();

    let mut curr = Some(Location::starting_location());
    while let Some(loc) = curr {
        println!("{:?}", loc);
        // log.log(&format!("{:?}", loc));
        curr = loc.rand_connection(&mut rng);
    }

    let mut c1 = Creature::new(s!("Jilly"), Species::Human);
    c1.place(Location::starting_location());

    while let Some(loc) = c1.location() {
        println!("{} is at {:?}", c1.name(), loc);
        c1.move_randomly(&mut rng);
    }

    let mut c2 = Creature::new(s!("Heather"), Species::Human);
    c2.place(Location::starting_location());

    while let Some(loc) = c2.location() {
        println!("{} is at {:?}", c2.name(), loc);
        c2.move_randomly(&mut rng);
    }

    println!("BEFORE INTERACTION: \n  {:?}\n  {:?}", c1, c2);
    creatures::interact(&mut c1, &mut c2, &mut rng);
    println!("AFTER INTERACTION: \n  {:?}\n  {:?}", c1, c2);

    puts!("===FIN 1===");
    // puts!(log);
}

impl Disposed for Creature<Location> {
    fn disposition(&self) -> Disposition {
        Disposition::Friendly
    }
}

impl DefaultHP for Creature<Location> {
    fn default_hp(&self) -> HP {
        self.species.default_hp()
    }
    fn hp(&self) -> HP {
        self.hp
    }
    fn increase_hp(&mut self, _by: HP) -> HP {
        unimplemented!();
        // self.hp = HP(self.hp.0 + by.0);
        // self.hp()
    }
    fn decrease_hp(&mut self, _by: HP) -> HP {
        unimplemented!();

        // self.hp = HP(self.hp.0 - by.0);
        // self.hp()
    }
}
