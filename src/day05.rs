use std::str::FromStr;

pub type Output = String;

enum Isa {
    CrateMover9000,
    CrateMover9001,
}

impl Isa {
    fn exec(self, state: &mut Stacks, cmd: Command) {
        match cmd {
            x @ Command::Move { .. } => self.mov(state, x),
        }
    }

    fn mov(self, state: &mut Stacks, Command::Move { amt, src, dst }: Command) {
        //println!("------------------------");
        //println!("MOV {amount} {source} -> {destination}");
        //println!("Before\n{:#?}", self);

        let Ok([s, d]) = state.0.get_many_mut([src - 1, dst - 1]) else {
            panic!("Invalid source/destination");
        };

        //let start_idx = s.len().saturating_sub(amount);
        let start_idx = s.len() - amt;

        match self {
            Self::CrateMover9000 => d.extend(s.drain(start_idx..).rev()),
            Self::CrateMover9001 => d.extend(s.drain(start_idx..)),
        };

        //println!("After\n{:#?}", self);
    }
}

#[derive(Debug)]
enum Command {
    Move { amt: usize, src: usize, dst: usize },
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("move ") {
            let (amt, s) = s.split_once(' ').unwrap();
            let amt = amt.parse().unwrap();

            let Some(s) = s.strip_prefix("from ") else {
                panic!("Invalid command arguments");
            };

            let (src, dst) = s.split_once(" to ").unwrap();

            let src = src.parse().unwrap();
            let dst = dst.parse().unwrap();

            Ok(Self::Move { amt, src, dst })
        } else {
            panic!("Unknown command")
        }
    }
}

#[derive(Debug)]
struct Stacks(Vec<Vec<u8>>);

impl Stacks {
    fn tops(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(self.0.iter().filter_map(|s| s.last().copied()).collect())
        }
    }
}

impl FromStr for Stacks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();

        let num = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut stacks = vec![Vec::new(); num];

        for l in lines {
            let mut index = 0;
            for c in l.split('[') {
                let s = c.trim_start();

                index += (c.len() - s.len()) / 4;

                if let Some(x @ b'A'..=b'Z') = s.as_bytes().first() {
                    stacks[index].push(*x);
                    index += 1;
                };

                let x = s.trim_end();

                index += (s.len() - x.len()) / 4;
            }
        }

        Ok(Self(stacks))
    }
}

fn task_one(input: &str) -> Output {
    let (state, commands) = input.split_once("\n\n").unwrap();

    let mut state: Stacks = state.parse().unwrap();
    let commands: Vec<Command> = commands.lines().map(|c| c.parse().unwrap()).collect();

    for c in commands {
        Isa::CrateMover9000.exec(&mut state, c);
    }

    state.tops()
}

fn task_two(input: &str) -> Output {
    let (state, commands) = input.split_once("\n\n").unwrap();

    let mut state: Stacks = state.parse().unwrap();
    let commands: Vec<Command> = commands.lines().map(|c| c.parse().unwrap()).collect();

    for c in commands {
        Isa::CrateMover9001.exec(&mut state, c);
    }

    state.tops()
}

#[cfg(test)]
mod day05 {
    use super::*;
    use crate::aoc_input;

    const INPUT: &'static str = aoc_input!(5);

    const EXAMPLE: &'static str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn task_one_example() {
        let result = task_one(EXAMPLE);
        assert_eq!("CMZ", result);
    }

    #[test]
    fn task_one_verify() {
        let result = task_one(INPUT);
        assert_eq!("LBLVVTVLP", result);
    }

    #[test]
    fn task_two_example() {
        let result = task_two(EXAMPLE);
        assert_eq!("MCD", result);
    }

    #[test]
    fn task_two_verify() {
        let result = task_two(INPUT);
        assert_eq!("TPFFBDRJD", result);
    }
}
