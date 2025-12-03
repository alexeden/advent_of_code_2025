use day3::{digits_into_usize, index_of_max};
use std::env;

fn main() {
    let path = env::args().nth(1).expect("Input file path is required");
    let file = std::fs::read_to_string(path).expect("Failed to read file");

    let sum = file
        .trim()
        .lines()
        .map(|line| {
            let len = line.len();
            let digits = line.chars().map(|c| c.to_digit(10).expect("Invalid digit"));
            let (max_index, first_max) = index_of_max(digits.clone().take(len - 1)).unwrap();
            let (_, last_max) = index_of_max(digits.clone().skip(max_index + 1)).unwrap();
            vec![first_max, last_max]
        })
        .map(|digits| digits_into_usize(digits).expect("Invalid number"))
        .sum::<usize>();

    println!("{:?}", sum);
}
