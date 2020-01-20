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

mod game1;
mod game2;

fn main() {
    puts!("===GAME 1 ===");

    game1::play();
    puts!("===GAME 2 ===");

    game2::play()
}
