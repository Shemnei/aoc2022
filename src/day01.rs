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

    calories.sort_unstable();

    calories.iter().rev().take(3).sum()
}

#[cfg(test)]
mod day01 {
    use super::*;
    use crate::aoc_input;

    const INPUT: &'static str = aoc_input!(1);

    const EXAMPLE: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn task_one_example() {
        let result = task_one(EXAMPLE);
        assert_eq!(24000, result);
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
    fn task_two_verify() {
        let result = task_two(INPUT);
        assert_eq!(201524, result);
    }
}
