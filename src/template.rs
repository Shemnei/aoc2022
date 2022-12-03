pub type Output = ();

fn task_one(input: &str) -> Output {
    todo!()
}

fn task_two(input: &str) -> Output {
    todo!()
}

#[cfg(test)]
mod day@@ {
    use super::*;
    use crate::aoc_input;

    const INPUT: &'static str = aoc_input!(@@);

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
