pub fn turn_left(d: u16, steps: u16) -> u16 {
    (d + 100 - steps % 100) % 100
}

pub fn turn_right(d: u16, steps: u16) -> u16 {
    (d + (steps % 100)) % 100
}

pub fn turn_left_with_passes(d: u16, steps: u16) -> (u16, u16) {
    (
        turn_left(d, steps),
        ((steps + 100 - d) / 100u16).saturating_sub(if d == 0 { 1 } else { 0 }),
    )
}

pub fn turn_right_with_passes(d: u16, steps: u16) -> (u16, u16) {
    (turn_right(d, steps), (steps + d) / 100u16)
}

pub struct Turn(pub Direction, pub u16);
pub enum Direction {
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

    #[test]
    fn test_turn_left_with_passes() {
        assert_eq!(turn_left_with_passes(10, 20), (90, 1));
        assert_eq!(turn_left_with_passes(10, 120), (90, 2));
        assert_eq!(turn_left_with_passes(1, 100), (1, 1));
        assert_eq!(turn_left_with_passes(0, 1), (99, 0));
        assert_eq!(turn_left_with_passes(0, 101), (99, 1));
        assert_eq!(turn_left_with_passes(0, 99), (1, 0));
        assert_eq!(turn_left_with_passes(50, 50), (0, 1));
        assert_eq!(turn_left_with_passes(25, 150), (75, 2));
        assert_eq!(turn_left_with_passes(25, 1050), (75, 11));
        assert_eq!(turn_left_with_passes(50, 1000), (50, 10));
        assert_eq!(turn_left_with_passes(50, 1050), (0, 11));
        assert_eq!(turn_left_with_passes(50, 1075), (75, 11));
    }

    #[test]
    fn test_turn_right_with_passes() {
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
