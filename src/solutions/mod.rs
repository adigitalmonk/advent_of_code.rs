mod day1;
mod day2;
mod day3;
mod day4;

use crate::prelude::*;
const ITERATIONS: u32 = 100;

pub fn run_day1() -> (u32, u32, Benchmark, Benchmark) {
    let day1_data = read_data("day1/full.txt");
    let p1_result = day1::part1(&day1_data);
    let p2_result = day1::part2(&day1_data);

    let bm_p1 = Benchmark::run(
        &|| {
            day1::part1(&day1_data);
        },
        ITERATIONS,
    );
    let bm_p2 = Benchmark::run(
        &|| {
            day1::part2(&day1_data);
        },
        ITERATIONS,
    );

    (p1_result, p2_result, bm_p1, bm_p2)
}

pub fn run_day2() -> (u32, u32, Benchmark, Benchmark) {
    let day2_data = read_data("day2/full.txt");
    let p1_result = day2::part1(&day2_data);
    let p2_result = day2::part2(&day2_data);

    let bm_p1 = Benchmark::run(
        &|| {
            day2::part1(&day2_data);
        },
        ITERATIONS,
    );
    let bm_p2 = Benchmark::run(
        &|| {
            day2::part2(&day2_data);
        },
        ITERATIONS,
    );

    (p1_result, p2_result, bm_p1, bm_p2)
}

pub fn run_day3() -> (u32, u32, Benchmark, Benchmark) {
    let day3_data = read_data("day3/full.txt");
    let p1_result = day3::part1(&day3_data);
    let p2_result = day3::part2(&day3_data);

    let bm_p1 = Benchmark::run(
        &|| {
            day3::part1(&day3_data);
        },
        ITERATIONS,
    );
    let bm_p2 = Benchmark::run(
        &|| {
            day3::part2(&day3_data);
        },
        ITERATIONS,
    );

    (p1_result, p2_result, bm_p1, bm_p2)
}

pub fn run_day4() -> (u32, u32, Benchmark, Benchmark) {
    let day4_data = read_data("day4/full.txt");
    let p1_result = day4::part1(&day4_data);
    let p2_result = day4::part2(&day4_data);

    let bm_p1 = Benchmark::run(
        &|| {
            day4::part1(&day4_data);
        },
        ITERATIONS,
    );
    let bm_p2 = Benchmark::run(
        &|| {
            day4::part2(&day4_data);
        },
        ITERATIONS,
    );

    (p1_result, p2_result, bm_p1, bm_p2)
}
