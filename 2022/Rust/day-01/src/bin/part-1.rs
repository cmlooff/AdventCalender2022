use day_01::process_part1;
use std::fs; // Using std::fs to read the input.txt into a string

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file)); // Applying process_part1 onto the file string
}

/** Elves Calories
 * First Elf:   1000, 2000, 3000 =  6000
 * Second Elf:  4000             =  4000
 * Third Elf:   5000, 6000       =  11,000
 * Fourth Elf:  7000, 8000, 9000 =  24,000
 * Fifth Elf:   10,000           =  10,000
 */

/** Problem Statement
 * In case the Elves get hungry and need extra snacks, they need to know which Elf to ask:
 * they'd like to know how many Calories are being carried by the Elf carrying the most Calories.
 * In the example above, this is 24000 (carried by the fourth Elf).
 * Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
 */