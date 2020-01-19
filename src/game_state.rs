use crate::locations::Locations;
// use crate::s;
use rand::prelude::ThreadRng;
// use rand::seq::SliceRandom;
// use std::collections::HashMap;

#[derive(Debug)]
pub struct GameState {
    locations: Locations,
    rng: ThreadRng,
}

// impl<'a> GameState<'a> {
//     pub fn create_random() -> GameState<'a> {
//         let mut rooms = Rooms::new();
//         rooms.add(Room::new("lawn", "The Front lawn"));
//         rooms.add(Room::new("porch", "The Porch"));
//         rooms.add(Room::new("driveway", "The Driveway"));

//         let mut map = MapData::new();
//         map.insert(s!["lawn"], vec![s!("driveway"), s!("porch")]);
//         map.insert(s!["driveway"], vec![s!("lawn"), s!("porch")]);
//         map.insert(s!["porch"], vec![s!("driveway")]);

//         GameState {
//             rooms,
//             map,
//             rng: rand::thread_rng(),
//         }
//     }

//     pub fn start_at(&self) -> Option<&Room> {
//         match self.map.keys().nth(0) {
//             Some(room_id) => self.rooms.find(room_id),
//             None => None,
//         }
//     }

//     pub fn rand_room(&mut self, from: &String) -> Option<&Room> {
//         let options = self.map.get(from).unwrap();
//         if let Some(room_id) = options.choose(&mut self.rng) {
//             self.rooms.find(room_id)
//         } else {
//             None
//         }
//     }
// }
// type MapData<'a> = HashMap<String, Vec<String>>;

// pub fn create_rooms() -> Vec<Room> {
//     vec![
//         Room {
//             desc: s!("Front lawn"),
//         },
//         Room { desc: s!("Porch") },
//         Room {
//             desc: s!("Driveway"),
//         },
//     ]
// }

// fn create_map(rooms: &[Room]) -> MapData {
//     let r1 = rooms.get(0).unwrap();
//     let r2 = rooms.get(1).unwrap();
//     let r3 = rooms.get(2).unwrap();

//     let mut map = HashMap::new();
//     map.insert(r1, vec![r2, r3]);
//     map.insert(r2, vec![r1, r3]);
//     map.insert(r3, vec![r1]);
//     map
// }
