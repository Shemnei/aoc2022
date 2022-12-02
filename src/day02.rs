use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn c(a: &Rps, b: &Rps) -> Option<Ordering> {
            match (self, other) {
                (Self::Rock, Self::Paper) => Some(Ordering::Less),
                (Self::Rock, Self::Scissors) => Some(Ordering::Greater),
                (Self::Paper, Self::Scissors) => Some(Ordering::Less),
                _ => None,
            }
        }

        c(self, other)
            .or_else(|| c(other, self).map(|o| o.reverse()))
            .or(Some(Ordering::Equal))
    }
}

fn task_one(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .filter_map(|num| num.parse::<u64>().ok())
                .sum()
        })
        .max()
        .unwrap()
}

fn task_two(input: &str) -> u64 {
    let mut calories = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .filter_map(|num| num.parse::<u64>().ok())
                .sum()
        })
        .collect::<Vec<_>>();

    (&mut calories).sort_unstable();

    calories.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_input;

    const INPUT: &'static str = aoc_input!(2);

    const EXAMPLE: &'static str = "A Y
B X
C Z";

    #[test]
    fn task_one_example() {
        let result = task_one(EXAMPLE);
        assert_eq!(24000, result);
    }

    #[test]
    fn task_one_print() {
        let result = task_one(INPUT);
        println!("D1T1: {result}");
    }

    #[test]
    fn task_one_verify() {
        let result = task_one(INPUT);
        assert_eq!(69281, result);
    }

    #[test]
    fn task_two_example() {
        let result = task_two(EXAMPLE);
        assert_eq!(45000, result);
    }

    #[test]
    fn task_two_print() {
        let result = task_two(INPUT);
        println!("D1T2: {result}");
    }

    #[test]
    fn task_two_verify() {
        let result = task_two(INPUT);
        assert_eq!(201524, result);
    }
}
