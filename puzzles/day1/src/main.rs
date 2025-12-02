#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dial {
    value: u16,
    /// Track the number of times the dial has hit zero
    zeros: u16,
}
impl Dial {
    pub fn new(value: u16) -> Self {
        Self { value, zeros: 0 }
    }
}

impl Dial {
    pub fn apply_turn(self, r: Turn) -> Self {
        let value = match r {
            Turn::Left(x) => (self.value + (100 - x)) % 100,
            Turn::Right(x) => (self.value + x) % 100,
        };

        Self {
            value,
            zeros: self.zeros + if value == 0 { 1 } else { 0 },
        }
    }

    pub fn zeros(&self) -> u16 {
        self.zeros
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Turn {
    Left(u16),
    Right(u16),
}

impl TryFrom<&str> for Turn {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (direction, steps) = value.split_at(1);
        let steps: u16 = steps.parse()?;
        match direction {
            "L" => Ok(Turn::Left(steps)),
            "R" => Ok(Turn::Right(steps)),
            _ => Err(anyhow::anyhow!("Invalid direction {}", direction)),
        }
    }
}

// impl From<Turn> for i16 {
//     fn from(value: Turn) -> Self {
//         match value {
//             Turn::Left(v) => 0i16 - v as i16,
//             Turn::Right(v) => v as i16,
//         }
//     }
// }

fn main() {
    let turns: Vec<Turn> = vec![];

    let dial: Dial = turns
        .into_iter()
        .fold(Dial::new(0), |d, turn| d.apply_turn(turn));

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn turn_dial_right() {
    //     assert_eq!(Dial::new(0), Dial::new(0).apply_turn(Turn::Right(100)));
    //     assert_eq!(Dial::new(1), Dial::new(0).apply_turn(Turn::Right(101)));
    //     assert_eq!(Dial::new(99), Dial::new(0).apply_turn(Turn::Right(99)));
    // }

    // #[test]
    // fn turn_dial_left() {
    //     assert_eq!(Dial::new(0), Dial::new(0).apply_turn(Turn::Left(100)));
    //     assert_eq!(Dial::new(99), Dial::new(0).apply_turn(Turn::Left(1)));
    //     assert_eq!(Dial::new(99), Dial::new(99).apply_turn(Turn::Left(100)));
    // }

    #[test]
    fn test_rotation_try_from() {
        assert_eq!(Turn::try_from("L50").unwrap(), Turn::Left(50));
        assert_eq!(Turn::try_from("R50").unwrap(), Turn::Right(50));
    }
}
