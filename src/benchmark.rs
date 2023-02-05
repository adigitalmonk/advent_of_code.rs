use std::time::{Duration, SystemTime};

pub mod prelude {
    pub use super::Benchmark;
}

pub struct Benchmark {
    pub average: Duration,
    pub fastest: Duration,
    pub slowest: Duration,
}

impl Benchmark {
    fn execute(func: &impl Fn(), iterations: u32) -> Vec<Duration> {
        let mut results: Vec<Duration> = Vec::new();
        for _ in 0..iterations {
            let started_at = SystemTime::now();
            func();
            let elapsed = started_at.elapsed().expect("Couldn't check elapsed");
            results.push(elapsed);
        }

        results
    }

    pub fn run(func: &impl Fn(), iterations: u32) -> Self {
        let mut results = Benchmark::execute(func, iterations);
        results.sort();

        let number_of_entries = results.len().try_into().expect("wasn't a safe number");
        let mut sum = Duration::new(0, 0);
        for num in &results {
            sum += *num;
        }
        let average = sum / number_of_entries;
        let slowest = *results.iter().last().expect("should be a duration");
        let fastest = *results.get(0).expect("should be duration");

        Self {
            average,
            fastest,
            slowest,
        }
    }
}
