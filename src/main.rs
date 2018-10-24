#[macro_use]
extern crate nom;

use std::env;

mod trace;

use trace::read_register_writes;

fn main() {
    let filename = env::args().nth(1).expect("Set a filename");
    read_register_writes(&filename).expect("Failed to read lines");
}
