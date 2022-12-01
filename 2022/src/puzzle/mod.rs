use std::fs;

mod day_one;

use day_one::DayOne;

pub trait Puzzle {
    fn part_one(&mut self) -> String;
    fn part_two(&mut self) -> String;
}

pub fn get(day: &str) -> Option<impl Puzzle> {
    match day {
        "1" => Some(DayOne::new(parse_input(day))),
        _ => None,
    }
}

fn parse_input(day: &str) -> String {
    let file = format!("input/day{}.txt", day);
    fs::read_to_string(file).unwrap()
}
