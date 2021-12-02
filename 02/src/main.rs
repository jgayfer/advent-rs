use std::str::FromStr;

enum Direction {
    FORWARD,
    UP,
    DOWN,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::FORWARD),
            "down" => Ok(Direction::DOWN),
            "up" => Ok(Direction::UP),
            _ => Err(()),
        }
    }
}

struct Command {
    direction: Direction,
    magnitude: i32,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let (direction_str, magnitude_str) = input.split_once(" ").unwrap();
        let direction = Direction::from_str(direction_str).unwrap();
        let magnitude = magnitude_str.parse::<i32>().unwrap();
        Ok(Command {
            direction,
            magnitude,
        })
    }
}

#[derive(Default)]
struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl Position {
    fn product(&self) -> i32 {
        self.depth * self.horizontal
    }
}

fn main() {
    let commands = parse_commands();

    println!("Part 1: {}", part_one(&commands));
    println!("Part 2: {}", part_two(&commands));
}

fn part_one(commands: &Vec<Command>) -> i32 {
    let mut position = Position::default();

    for command in commands {
        match command.direction {
            Direction::FORWARD => position.horizontal += command.magnitude,
            Direction::UP => position.depth -= command.magnitude,
            Direction::DOWN => position.depth += command.magnitude,
        }
    }

    position.product()
}

fn part_two(commands: &Vec<Command>) -> i32 {
    let mut position = Position::default();

    for command in commands {
        match command.direction {
            Direction::FORWARD => {
                position.horizontal += command.magnitude;
                position.depth += position.aim * command.magnitude;
            }
            Direction::UP => position.aim -= command.magnitude,
            Direction::DOWN => position.aim += command.magnitude,
        }
    }

    position.product()
}

fn parse_commands() -> Vec<Command> {
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .map(|line| Command::from_str(line).unwrap())
        .collect()
}
