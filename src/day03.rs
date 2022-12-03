use std::collections::HashSet;

pub type Output = u64;

fn find_item(items: &[&str]) -> u8 {
    items
        .iter()
        .map(|i| HashSet::<u8>::from_iter(i.bytes()))
        .reduce(|acc, item| acc.intersection(&item).copied().collect())
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

fn priority(item: u8) -> u64 {
    match item {
        b'a'..=b'z' => 1 + (item - b'a') as u64,
        b'A'..=b'Z' => 27 + (item - b'A') as u64,
        _ => panic!("Invalid item `{item}`"),
    }
}

fn task_one(input: &str) -> Output {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            priority(find_item(&[a, b]))
        })
        .sum()
}

fn task_two(input: &str) -> Output {
    input
        .lines()
        .array_chunks()
        .map(|a: [&str; 3]| priority(find_item(&a)))
        .sum()
}

#[cfg(test)]
mod day03 {
    use super::*;
    use crate::aoc_input;

    const INPUT: &'static str = aoc_input!(3);

    const EXAMPLE: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn task_one_example() {
        let result = task_one(EXAMPLE);
        assert_eq!(157, result);
    }

    #[test]
    fn task_one_verify() {
        let result = task_one(INPUT);
        assert_eq!(7766, result);
    }

    #[test]
    fn task_two_example() {
        let result = task_two(EXAMPLE);
        assert_eq!(70, result);
    }

    #[test]
    fn task_two_verify() {
        let result = task_two(INPUT);
        assert_eq!(2415, result);
    }
}
