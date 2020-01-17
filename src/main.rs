extern crate rand;

mod creatures;
mod map;
mod rooms;
mod s_macro;

fn main() {
    println!("creating map");
    let mut rng = rand::thread_rng();

    let rooms = map::create_rooms();
    let map = map::create_map(&rooms);

    let mut maybe_room = rooms.get(0);
    println!("Starting at {:?}", maybe_room);
    for _ in 1..15 {
        if let Some(curr) = maybe_room {
            maybe_room = map::rand_room(&map, &curr, &mut rng);
            println!("Moved to {:?}", maybe_room);
        }
    }
}
