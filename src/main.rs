#![deny(clippy::pedantic)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::suspicious)]
#![deny(clippy::complexity)]
#![warn(clippy::style)]

use std::{fmt::Display, time::Duration};

mod loader;
mod solutions;

mod prelude {
    pub use crate::loader::*;
}

fn friendly<T>(day: u32, (p1, p2, p1d, p2d): (T, T, Duration, Duration))
where
    T: Display,
{
    println!("Day {day} Part 1 :: Solution {p1} :: Duration {p1d:?}");
    println!("Day {day} Part 2 :: Solution {p2} :: Duration {p2d:?}");
}

fn main() {
    friendly(1, solutions::run_day1());
    friendly(2, solutions::run_day2());
    friendly(3, solutions::run_day3());
    friendly(4, solutions::run_day4());
    friendly(5, solutions::run_day5());
    friendly(6, solutions::run_day6());
}
