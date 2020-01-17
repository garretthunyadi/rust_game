use crate::rooms::Room;
use crate::s;
use rand::seq::SliceRandom;
use std::collections::HashMap; // 0.7.2

pub fn create_rooms() -> Vec<Room> {
    vec![
        Room {
            desc: s!("Front lawn"),
        },
        Room { desc: s!("Porch") },
        Room {
            desc: s!("Driveway"),
        },
    ]
}

pub type Map<'a> = HashMap<&'a Room, Vec<&'a Room>>;

pub fn create_map(rooms: &[Room]) -> Map {
    let r1 = rooms.get(0).unwrap();
    let r2 = rooms.get(1).unwrap();
    let r3 = rooms.get(2).unwrap();

    let mut map = HashMap::new();
    map.insert(r1, vec![r2, r3]);
    map.insert(r2, vec![r1, r3]);
    map.insert(r3, vec![r1]);
    map
}

pub fn rand_room<'a>(
    map: &'a Map,
    room: &Room,
    rng: &mut rand::prelude::ThreadRng,
) -> Option<&'a Room> {
    let options = map.get(room).unwrap();
    if let Some(res) = options.choose(rng) {
        Some(res)
    } else {
        None
    }
}
