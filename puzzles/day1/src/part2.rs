use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn turn_left_with_passes(d: u16, steps: u16) -> (u16, u16) {
    let next = (d + 100 - steps % 100) % 100;
    let loops = steps / 100u16;
    let passes = if next == 0 {
        1 + loops.saturating_sub(1)
    } else {
        loops + if next > d && d != 0 { 1 } else { 0 }
    };

    (next, passes)
}

fn turn_right_with_passes(d: u16, steps: u16) -> (u16, u16) {
    let next = (d + (steps % 100)) % 100;
    let loops = steps / 100u16;
    let passes = if next == 0 {
        1 + loops.saturating_sub(1)
    } else {
        loops + if next < d && d != 0 { 1 } else { 0 }
    };

    (next, passes)
}

// fn calc_zero_passes(d: u16, steps: u16) -> u16 {
//     let next = (d + 100 - steps % 100) % 100;
//     let loops = steps / 100u16;
//     let passes = if next == 0 {
//         1 + loops.saturating_sub(1)
//     } else {
//         loops + if next > d && d != 0 { 1 } else { 0 }
//     };

struct Turn(Direction, u16);

enum Direction {
    Left,
    Right,
}

impl TryFrom<&str> for Turn {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, steps) = value.split_at(1);
        let dir = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(anyhow::anyhow!("Invalid direction {}", dir)),
        };
        Ok(Self(dir, steps.parse()?))
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_turn_left_with_passes() {
    //     assert_eq!(turn_left_with_passes(0, 1), (99, 0));
    //     assert_eq!(turn_left_with_passes(0, 100), (0, 1));
    //     assert_eq!(turn_left_with_passes(0, 101), (99, 1));
    //     assert_eq!(turn_left_with_passes(0, 99), (1, 0));
    //     assert_eq!(turn_left_with_passes(50, 50), (0, 1));
    //     assert_eq!(turn_left_with_passes(25, 150), (75, 2));
    //     assert_eq!(turn_left_with_passes(25, 1050), (75, 11));
    //     assert_eq!(turn_left_with_passes(50, 1000), (50, 10));
    //     assert_eq!(turn_left_with_passes(50, 1050), (0, 11));
    //     assert_eq!(turn_left_with_passes(50, 1075), (75, 11));
    // }

    #[test]
    fn test_turn_right() {
        assert_eq!(turn_right_with_passes(0, 100), (0, 1));
        assert_eq!(turn_right_with_passes(99, 1), (0, 1));
        assert_eq!(turn_right_with_passes(0, 101), (1, 1));
        assert_eq!(turn_right_with_passes(0, 99), (99, 0));
        assert_eq!(turn_right_with_passes(50, 50), (0, 1));
        assert_eq!(turn_right_with_passes(25, 1050), (75, 10));
        assert_eq!(turn_right_with_passes(50, 1000), (50, 10));
        assert_eq!(turn_right_with_passes(50, 1050), (0, 11));
        assert_eq!(turn_right_with_passes(50, 1075), (25, 11));
    }
}
