use day3::{digits_into_usize, extract_max_digits};
use std::env;

fn main() {
    let path = env::args().nth(1).expect("Input file path is required");
    let file = std::fs::read_to_string(path).expect("Failed to read file");

    let sum = file
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit"))
                .collect::<Vec<u32>>()
        })
        .map(|digits| extract_max_digits(digits, 12))
        .map(|digits| digits_into_usize(digits).expect("Invalid number"))
        .sum::<usize>();

    println!("{:?}", sum);
}
