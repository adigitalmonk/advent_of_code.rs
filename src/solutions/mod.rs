mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use std::time::{Duration, SystemTime};

use crate::prelude::*;

pub fn run_day1() -> (u32, u32, Duration, Duration) {
    let data = read_data("day1/full.txt");

    let started_at = SystemTime::now();
    let p1_result = day1::part1(&data);
    let p1_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    let started_at = SystemTime::now();
    let p2_result = day1::part2(&data);
    let p2_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    (p1_result, p2_result, p1_elapsed, p2_elapsed)
}
pub fn run_day2() -> (u32, u32, Duration, Duration) {
    let data = read_data("day2/full.txt");

    let started_at = SystemTime::now();
    let p1_result = day2::part1(&data);
    let p1_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    let started_at = SystemTime::now();
    let p2_result = day2::part2(&data);
    let p2_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    (p1_result, p2_result, p1_elapsed, p2_elapsed)
}

pub fn run_day3() -> (u32, u32, Duration, Duration) {
    let data = read_data("day3/full.txt");

    let started_at = SystemTime::now();
    let p1_result = day3::part1(&data);
    let p1_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    let started_at = SystemTime::now();
    let p2_result = day3::part2(&data);
    let p2_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    (p1_result, p2_result, p1_elapsed, p2_elapsed)
}

pub fn run_day4() -> (u32, u32, Duration, Duration) {
    let data = read_data("day4/full.txt");

    let started_at = SystemTime::now();
    let p1_result = day4::part1(&data);
    let p1_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    let started_at = SystemTime::now();
    let p2_result = day4::part2(&data);
    let p2_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    (p1_result, p2_result, p1_elapsed, p2_elapsed)
}

pub fn run_day5() -> (String, String, Duration, Duration) {
    let data = read_data("day5/full.txt");

    let started_at = SystemTime::now();
    let p1_result = day5::part1(&data);
    let p1_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    let started_at = SystemTime::now();
    let p2_result = day5::part2(&data);
    let p2_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    (p1_result, p2_result, p1_elapsed, p2_elapsed)
}

pub fn run_day6() -> (usize, usize, Duration, Duration) {
    let data = read_all_data("day6/full.txt");

    let started_at = SystemTime::now();
    let p1_result = day6::part1(&data);
    let p1_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    let started_at = SystemTime::now();
    let p2_result = day6::part2(&data);
    let p2_elapsed = started_at.elapsed().expect("Couldn't check elapsed");

    (p1_result, p2_result, p1_elapsed, p2_elapsed)
}
