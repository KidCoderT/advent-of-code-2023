use std::fs::File;
use std::io::prelude::*;

pub fn read_input() -> String {
    let mut f = File::open("input.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer
}
