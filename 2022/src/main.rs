use std::env;

mod puzzle;

use puzzle::Puzzle;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_num = args.get(1).unwrap().as_str();

    let mut day = puzzle::get(day_num).unwrap();

    println!("Part one: {}", day.part_one());
    println!("Part two: {}", day.part_two());
}
