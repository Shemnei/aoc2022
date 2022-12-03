use std::cmp::Ordering;

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Rps {
    const fn shape_score(&self) -> u64 {
        *self as u64
    }
}

impl From<u8> for Rps {
    fn from(value: u8) -> Self {
        match value {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => panic!("Invalid byte {value}"),
        }
    }
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn _partial_cmp(a: &Rps, b: &Rps) -> Option<Ordering> {
            match (a, b) {
                (Rps::Rock, Rps::Paper) => Some(Ordering::Less),
                (Rps::Rock, Rps::Scissors) => Some(Ordering::Greater),
                (Rps::Paper, Rps::Scissors) => Some(Ordering::Less),
                _ => None,
            }
        }

        _partial_cmp(self, other)
            .or_else(|| Some(_partial_cmp(other, self).map_or(Ordering::Equal, Ordering::reverse)))
    }
}

fn score_round(a: Rps, b: Rps) -> u64 {
    b.shape_score()
        + if b > a {
            6
        } else if b == a {
            3
        } else {
            0
        }
}

fn task_one(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let b = l.as_bytes();
            score_round(Rps::from(b[0]), Rps::from(b[2]))
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoundOutcome {
    Lose,
    Draw,
    Win,
}

impl From<u8> for RoundOutcome {
    fn from(value: u8) -> Self {
        match value {
            b'X' => Self::Lose,
            b'Y' => Self::Draw,
            b'Z' => Self::Win,
            _ => panic!("Invalid byte {value}"),
        }
    }
}

impl Rps {
    const fn draw(self) -> Self {
        self
    }

    const fn win(self) -> Self {
        // Could also be index + 1  % 3
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    const fn lose(self) -> Self {
        // Could also be index - 1  % 3
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn from_outcome(self, outcome: RoundOutcome) -> Self {
        match outcome {
            RoundOutcome::Win => self.win(),
            RoundOutcome::Lose => self.lose(),
            RoundOutcome::Draw => self.draw(),
        }
    }
}

fn task_two(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let b = l.as_bytes();
            let h = Rps::from(b[0]);
            let o = RoundOutcome::from(b[2]);

            score_round(h, h.from_outcome(o))
        })
        .sum()
}

#[cfg(test)]
mod day02 {
    use super::*;
    use crate::aoc_input;

    const INPUT: &'static str = aoc_input!(2);

    const EXAMPLE: &'static str = "A Y
B X
C Z";

    #[test]
    fn task_one_example() {
        let result = task_one(EXAMPLE);
        assert_eq!(15, result);
    }

    #[test]
    fn task_one_verify() {
        let result = task_one(INPUT);
        assert_eq!(13268, result);
    }

    #[test]
    fn task_two_example() {
        let result = task_two(EXAMPLE);
        assert_eq!(12, result);
    }

    #[test]
    fn task_two_verify() {
        let result = task_two(INPUT);
        assert_eq!(15508, result);
    }
}
