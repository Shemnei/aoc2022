#![allow(dead_code)]
#![feature(iter_array_chunks)]
#![feature(get_many_mut)]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

#[macro_export]
macro_rules! aoc_input {
    ($day:literal) => {
        include_str!(concat!("../input/day", stringify!($day)))
    };
}
