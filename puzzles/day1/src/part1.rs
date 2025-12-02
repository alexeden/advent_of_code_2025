use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn turn_left(d: u16, steps: u16) -> u16 {
    (d + 100 - (steps % 100)) % 100
}

fn turn_right(d: u16, steps: u16) -> u16 {
    (d + (steps % 100)) % 100
}

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
            let next = match direction {
                Direction::Left => turn_left(d, steps),
                Direction::Right => turn_right(d, steps),
            };
            (next, zeros + if next == 0 { 1 } else { 0 })
        });

    println!("Password: {}", password);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_left() {
        assert_eq!(turn_left(0, 100), 0);
        assert_eq!(turn_left(0, 101), 99);
        assert_eq!(turn_left(0, 99), 1);
        assert_eq!(turn_left(50, 50), 0);
        assert_eq!(turn_left(25, 1050), 75);
    }

    #[test]
    fn test_turn_right() {
        assert_eq!(turn_right(0, 100), 0);
        assert_eq!(turn_right(0, 101), 1);
        assert_eq!(turn_right(0, 99), 99);
        assert_eq!(turn_right(50, 50), 0);
        assert_eq!(turn_right(25, 1050), 75);
    }
}
