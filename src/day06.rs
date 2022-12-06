use std::collections::HashSet;

pub type Output = usize;

fn task_one(input: &str) -> Output {
    input
        .as_bytes()
        .array_windows::<4>()
        .enumerate()
        .find(|(_, x)| HashSet::<u8>::from_iter(x.iter().copied()).len() == 4)
        .unwrap()
        .0
        + 4
}

fn task_two(input: &str) -> Output {
    input
        .as_bytes()
        .array_windows::<14>()
        .enumerate()
        .find(|(_, x)| HashSet::<u8>::from_iter(x.iter().copied()).len() == 14)
        .unwrap()
        .0
        + 14
}

#[cfg(test)]
mod day06 {
    use super::*;
    use crate::aoc_input;

    const INPUT: &'static str = aoc_input!(6);

    const EXAMPLE: &'static str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn task_one_example() {
        let result = task_one(EXAMPLE);
        assert_eq!(11, result);
    }

    #[test]
    fn task_one_verify() {
        let result = task_one(INPUT);
        assert_eq!(1987, result);
    }

    #[test]
    fn task_two_example() {
        let result = task_two(EXAMPLE);
        assert_eq!(26, result);
    }

    #[test]
    fn task_two_verify() {
        let result = task_two(INPUT);
        assert_eq!(3059, result);
    }
}
