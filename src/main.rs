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
}
