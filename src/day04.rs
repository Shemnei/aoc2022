pub type Output = u64;

fn parse_range(s: &str) -> (u64, u64) {
    let nums = s
        .split_once('-')
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .unwrap();

    (nums.0, nums.1)
}

fn parse_ranges(s: &str) -> ((u64, u64), (u64, u64)) {
    s.split_once(',')
        .map(|(a, b)| (parse_range(a), parse_range(b)))
        .unwrap()
}

fn task_one(input: &str) -> Output {
    input
        .lines()
        .filter(|l| {
            let ((aa, ab), (ba, bb)) = parse_ranges(l);

            (aa <= ba && ab >= bb) || (ba <= aa && bb >= ab)
        })
        .count() as u64
}

fn task_two(input: &str) -> Output {
    input
        .lines()
        .filter(|l| {
            let ((aa, ab), (ba, bb)) = parse_ranges(l);

            // Fully contains
            (aa <= ba && ab >= bb)
                || (ba <= aa && bb >= ab)
                // A contains b
                || (aa <= ba && ab >= ba)
                || (aa <= bb && ab >= bb)
                // B contains a
                || (ba <= aa && bb >= aa)
                || (ba <= ab && bb >= ab)
        })
        .count() as u64
}

#[cfg(test)]
mod day04 {
    use super::*;
    use crate::aoc_input;

    const INPUT: &'static str = aoc_input!(4);

    const EXAMPLE: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn task_one_example() {
        let result = task_one(EXAMPLE);
        assert_eq!(2, result);
    }

    #[test]
    fn task_one_verify() {
        let result = task_one(INPUT);
        assert_eq!(528, result);
    }

    #[test]
    fn task_two_example() {
        let result = task_two(EXAMPLE);
        assert_eq!(4, result);
    }

    #[test]
    fn task_two_verify() {
        let result = task_two(INPUT);
        assert_eq!(881, result);
    }
}
