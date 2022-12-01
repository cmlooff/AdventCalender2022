use day_01::process_part1;
use std::fs; // Using std::fs to read the input.txt into a string

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file)); // Applying process_part1 onto the file string
}
