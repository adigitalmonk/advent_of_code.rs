mod benchmark;
mod day1;
mod loader;

use benchmark::Benchmark;

fn run_day1() -> (u32, u32, Benchmark, Benchmark) {
    let day1_data = loader::read_data("day1/full.txt");
    let p1_result = day1::part1(&day1_data);
    let p2_result = day1::part2(&day1_data);

    let iterations = 1000;
    let bm_p1 = Benchmark::run(
        &|| {
            day1::part1(&day1_data);
        },
        iterations,
    );
    let bm_p2 = Benchmark::run(
        &|| {
            day1::part2(&day1_data);
        },
        iterations,
    );

    (p1_result, p2_result, bm_p1, bm_p2)
}

fn main() {
    let (d1p1, d1p2, bm_p1, bm_p2) = run_day1();
    println!(
        "Day 1 Part 1 :: Solution: {} :: Fastest {:?} :: Slowest: {:?} :: Average: {:?}",
        d1p1, bm_p1.fastest, bm_p1.slowest, bm_p1.average
    );
    println!(
        "Day 1 Part 2 :: Solution: {} :: Fastest {:?} :: Slowest: {:?} :: Average: {:?}",
        d1p2, bm_p2.fastest, bm_p2.slowest, bm_p2.average
    );
}
