use crate::puzzle::Puzzle;

pub struct DayOne {
    elves: Vec<Elf>,
}

impl DayOne {
    pub fn new(input: String) -> Self {
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
        self.elves
            .iter()
            .map(|elf| elf.calories())
            .max()
            .unwrap()
            .to_string()
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
