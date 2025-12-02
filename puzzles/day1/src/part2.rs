use day1::{Direction, Turn, turn_left_with_passes, turn_right_with_passes};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = env::args().nth(1).expect("Input file path is required");
    let file = File::open(path).expect("Failed to open file");

    let (_, password) = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|l| Turn::try_from(l.as_str()).expect("Invalid turn: {l}"))
        .fold((50_u16, 0), |(d, zeros), Turn(direction, steps)| {
            let (next, passes) = match direction {
                Direction::Left => turn_left_with_passes(d, steps),
                Direction::Right => turn_right_with_passes(d, steps),
            };
            (next, zeros + passes)
        });

    println!("Password: {}", password);
}
