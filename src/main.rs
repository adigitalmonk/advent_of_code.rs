mod benchmark;
mod loader;
mod solutions;

mod prelude {
    pub use crate::benchmark::prelude::*;
    pub use crate::loader::*;
}

fn main() {
    let (d1p1, d1p2, bm_p1, bm_p2) = solutions::run_day1();

    println!(
        "Day 1 Part 1 :: Solution: {} :: Fastest {:?} :: Slowest: {:?} :: Average: {:?}",
        d1p1, bm_p1.fastest, bm_p1.slowest, bm_p1.average
    );
    println!(
        "Day 1 Part 2 :: Solution: {} :: Fastest {:?} :: Slowest: {:?} :: Average: {:?}",
        d1p2, bm_p2.fastest, bm_p2.slowest, bm_p2.average
    );

    let (d2p1, d2p2, bm_p1, bm_p2) = solutions::run_day2();

    println!(
        "Day 2 Part 1 :: Solution: {} :: Fastest {:?} :: Slowest: {:?} :: Average: {:?}",
        d2p1, bm_p1.fastest, bm_p1.slowest, bm_p1.average
    );
    println!(
        "Day 2 Part 2 :: Solution: {} :: Fastest {:?} :: Slowest: {:?} :: Average: {:?}",
        d2p2, bm_p2.fastest, bm_p2.slowest, bm_p2.average
    );

    let (d3p1, d3p2, bm_p1, bm_p2) = solutions::run_day3();
    println!(
        "Day 3 Part 1 :: Solution: {} :: Fastest {:?} :: Slowest: {:?} :: Average: {:?}",
        d3p1, bm_p1.fastest, bm_p1.slowest, bm_p1.average
    );
    println!(
        "Day 3 Part 2 :: Solution: {} :: Fastest {:?} :: Slowest: {:?} :: Average: {:?}",
        d3p2, bm_p2.fastest, bm_p2.slowest, bm_p2.average
    );

    let (d4p1, d4p2, bm_p1, bm_p2) = solutions::run_day4();
    println!(
        "Day 4 Part 1 :: Solution: {} :: Fastest {:?} :: Slowest: {:?} :: Average: {:?}",
        d4p1, bm_p1.fastest, bm_p1.slowest, bm_p1.average
    );
    println!(
        "Day 4 Part 2 :: Solution: {} :: Fastest {:?} :: Slowest: {:?} :: Average: {:?}",
        d4p2, bm_p2.fastest, bm_p2.slowest, bm_p2.average
    );
}
