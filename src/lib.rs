#![allow(dead_code)]

pub mod day01;
pub mod day02;

#[macro_export]
macro_rules! aoc_input {
    ($day:literal) => {
        include_str!(concat!("../input/day", stringify!($day)))
    };
}
