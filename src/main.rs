use std::fs;
use std::error::Error;

fn main() {
    let message: String = fs::read_to_string("practice.txt").unwrap();
    println!("{}", message);
}
