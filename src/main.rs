extern crate rand;

mod creatures;
mod game_state;
mod rooms;
mod s_macro;

use game_state::GameState;

fn main() {
    let mut state = GameState::create_random();
    let mut maybe_room = state.start_at();

    println!("Starting at {:?}", maybe_room);
    // for _ in 1..15 {
    //     if let Some(curr) = maybe_room {
    //         maybe_room = state.rand_room(&curr.id);
    //         println!("Moved to {:?}", maybe_room);
    //     }
    // }
}
