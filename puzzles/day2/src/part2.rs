use day2::{has_repeating_substring, parse_range_or_panic};
use std::env;

fn main() {
    let path = env::args().nth(1).expect("Input file path is required");
    let file = std::fs::read_to_string(path).expect("Failed to read file");

    let sum = file
        .trim()
        .split(',')
        .flat_map(parse_range_or_panic)
        .filter(|num| has_repeating_substring(&num.to_string()))
        .sum::<usize>();

    println!("{:?}", sum);
}
