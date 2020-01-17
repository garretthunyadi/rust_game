extern crate rand;

use rand::seq::SliceRandom;
use std::collections::HashMap; // 0.7.2

macro_rules! s {
    ($e:expr) => {
        String::from($e)
    };
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Room {
    desc: String,
}

fn create_rooms() -> Vec<Room> {
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

type Map<'a> = HashMap<&'a Room, Vec<&'a Room>>;

fn create_map(rooms: &[Room]) -> Map {
    let r1 = rooms.get(0).unwrap();
    let r2 = rooms.get(1).unwrap();
    let r3 = rooms.get(2).unwrap();

    let mut map = HashMap::new();
    map.insert(r1, vec![r2, r3]);
    map.insert(r2, vec![r1, r3]);
    map.insert(r3, vec![r1]);
    map
}

fn rand_room<'a>(
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

fn main() {
    println!("creating map");
    let mut rng = rand::thread_rng();

    let rooms = create_rooms();
    let map = create_map(&rooms);

    let mut maybe_room = rooms.get(0);
    println!("START at {:?}", maybe_room);
    for _ in 1..15 {
        if let Some(curr) = maybe_room {
            maybe_room = rand_room(&map, &curr, &mut rng);
            println!("Moved to {:?}", maybe_room);
        }
    }
}
