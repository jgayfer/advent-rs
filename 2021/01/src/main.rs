fn main() {
    let input_nums = parse_input();
    println!("Part 1: {}", get_increase_count(&input_nums, 1));
    println!("Part 2: {}", get_increase_count(&input_nums, 3));
}

fn get_increase_count(input_nums: &Vec<i32>, window_size: usize) -> usize {
    return input_nums
        .windows(window_size)
        .map(|w| w.iter().sum())
        .fold((0, i32::MAX), |(count, prev), next: i32| {
            (if next > prev { count + 1 } else { count }, next)
        })
        .0;
}

fn parse_input() -> Vec<i32> {
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}
