use std::{fmt::Debug, time::Duration};

mod loader;
mod solutions;

mod prelude {
    pub use crate::loader::*;
}

fn friendly(day: usize, part: usize, solution: &impl Debug, duration: Duration) {
    println!("Day {day} Part {part} :: Solution {solution:?} :: Duration {duration:?}");
}

fn main() {
    let (p1, p2, p1d, p2d) = solutions::run_day1();
    friendly(1, 1, &p1, p1d);
    friendly(1, 2, &p2, p2d);

    let (p1, p2, p1d, p2d) = solutions::run_day2();
    friendly(2, 1, &p1, p1d);
    friendly(2, 2, &p2, p2d);

    let (p1, p2, p1d, p2d) = solutions::run_day3();
    friendly(3, 1, &p1, p1d);
    friendly(3, 2, &p2, p2d);

    let (p1, p2, p1d, p2d) = solutions::run_day4();
    friendly(4, 1, &p1, p1d);
    friendly(4, 2, &p2, p2d);

    let (p1, p2, p1d, p2d) = solutions::run_day5();
    friendly(5, 1, &p1, p1d);
    friendly(5, 2, &p2, p2d);

    let (p1, p2, p1d, p2d) = solutions::run_day6();
    friendly(6, 1, &p1, p1d);
    friendly(6, 2, &p2, p2d);
}
