use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Dial(u16);

impl Add<u16> for Dial {
    type Output = Dial;
    fn add(self, rhs: u16) -> Self::Output {
        Dial((self.0 + rhs) % 100)
    }
}

impl Sub<u16> for Dial {
    type Output = Dial;
    fn sub(self, rhs: u16) -> Self::Output {
        Dial((self.0 + 100 - (rhs) % 100) % 100)
    }
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

    let (_, password): (Dial, u16) = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|l| Turn::try_from(l.as_str()).expect("Invalid turn: {l}"))
        .fold((Dial(50), 0), |(d, zeros), Turn(direction, steps)| {
            let next = match direction {
                Direction::Left => d - steps,
                Direction::Right => d + steps,
            };
            (next, zeros + if next.0 == 0 { 1 } else { 0 })
        });

    println!("Password: {}", password);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_dial_right() {
        assert_eq!(Dial(0) + 100, Dial(0));
        assert_eq!(Dial(0) + 101, Dial(1));
        assert_eq!(Dial(0) + 99, Dial(99));
        assert_eq!(Dial(50) + 50, Dial(0));
        assert_eq!(Dial(25) + 1050, Dial(75));
    }

    #[test]
    fn turn_dial_left() {
        assert_eq!(Dial(0) - 100, Dial(0));
        assert_eq!(Dial(0) - 101, Dial(99));
        assert_eq!(Dial(0) - 99, Dial(1));
        assert_eq!(Dial(50) - 50, Dial(0));
        assert_eq!(Dial(25) - 1050, Dial(75));
    }
}
