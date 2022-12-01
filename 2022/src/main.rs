use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args.get(1).unwrap().as_str();

    let mut puzzle = get_puzzle(day).unwrap();

    println!("Part one: {}", puzzle.part_one());
    println!("Part two: {}", puzzle.part_two());
}

fn get_puzzle(day: &str) -> Option<impl Puzzle> {
    match day {
        "1" => Some(DayOne::new(parse_input(day))),
        _ => None,
    }
}

fn parse_input(day: &str) -> String {
    let file = format!("input/day{}.txt", day);
    fs::read_to_string(file).unwrap()
}

trait Puzzle {
    fn part_one(&mut self) -> String;
    fn part_two(&mut self) -> String;
}

struct DayOne {
    pub elves: Vec<Elf>,
}

impl DayOne {
    fn new(input: String) -> Self {
        Self {
            elves: input
                .split("\n\n")
                .map(|foods| Elf {
                    food: foods
                        .trim()
                        .split("\n")
                        .into_iter()
                        .map(|food| food.parse::<u32>().unwrap())
                        .collect(),
                })
                .collect(),
        }
    }
}

impl Puzzle for DayOne {
    fn part_one(&mut self) -> String {
        self.elves.iter().map(|elf| elf.calories()).max().unwrap().to_string()
    }

    fn part_two(&mut self) -> String {
        let mut calories: Vec<u32> = self.elves.iter().map(|elf| elf.calories()).collect();
        calories.sort();
        calories.iter().rev().take(3).sum::<u32>().to_string()
    }
}

struct Elf {
    pub food: Vec<u32>,
}

impl Elf {
    fn calories(&self) -> u32 {
        self.food.iter().sum()
    }
}
