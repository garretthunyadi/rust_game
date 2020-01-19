use crate::puts;
use crate::rand::Rng;

pub fn maybe<F: Fn()>(rng: &mut rand::prelude::ThreadRng, f: F) {
    if rng.gen() {
        f();
    } else {
        puts!("<shhh..>");
    }
}
