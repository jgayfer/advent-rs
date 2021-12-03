use std::ops::Range;

fn main() {
    let inputs = parse_input();
    let bit_range = bit_range(&inputs);

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for bit in bit_range {
       gamma.push(most_common(&inputs, bit));
       epsilon.push(least_common(&inputs, bit));
    }


    let gamma_int = i32::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon_int = i32::from_str_radix(epsilon.as_str(), 2).unwrap();
    let power_rate = gamma_int * epsilon_int;

    println!("Part one: {}", power_rate);
}

fn bit_range(inputs: &Vec<String>) -> Range<usize> {
    0..inputs.iter().nth(0).unwrap().len()
}

fn most_common(inputs: &Vec<String>, index: usize) -> char {
    let zeroes = inputs.iter().filter(|input| input.chars().nth(index).unwrap() == '0').count();
    let ones = inputs.iter().filter(|input| input.chars().nth(index).unwrap() == '1').count();
    if zeroes > ones { '0' } else { '1' }
}

fn least_common(inputs: &Vec<String>, index: usize) -> char {
    let zeroes = inputs.iter().filter(|input| input.chars().nth(index).unwrap() == '0').count();
    let ones = inputs.iter().filter(|input| input.chars().nth(index).unwrap() == '1').count();
    if zeroes < ones { '0' } else { '1' }
}

fn parse_input() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(|line| line.to_string())
        .collect()
}
