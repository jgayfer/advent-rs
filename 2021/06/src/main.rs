fn main() {
    let mut buckets = vec![0; 9];

    parse_input()
        .into_iter()
        .for_each(|timer| buckets[timer] += 1);

    println!("Part 1: {:?}", fish_count(buckets.clone(), 80));
    println!("Part 2: {:?}", fish_count(buckets.clone(), 256));
}

fn fish_count(mut buckets: Vec<usize>, days: u32) -> usize {
    for _ in 0..days {
        buckets.rotate_left(1);
        let new_count = buckets[8];
        buckets[6] = buckets[6] + new_count;
        buckets[8] = new_count;
    }
    buckets.iter().sum()
}

fn parse_input() -> Vec<usize> {
    return include_str!("../input.txt")
        .trim()
        .split(",")
        .map(|entry| entry.parse().unwrap())
        .collect();
}
