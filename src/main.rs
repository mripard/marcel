#[macro_use] extern crate nom;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;

use std::env;

mod decoder;
mod error;
mod trace;

use decoder::Device;
use trace::read_register_writes;

fn main() {
    let offset = 0xf02d2000;

    let filename = env::args().nth(1).expect("Set a filename");
    let writes = read_register_writes(&filename).unwrap();

    let device_file = env::args().nth(2).expect("Set a device yaml file");
    let device = Device::new(&device_file, offset).unwrap();
    println!("{:?}", device);

    device.decode(&writes);
}
