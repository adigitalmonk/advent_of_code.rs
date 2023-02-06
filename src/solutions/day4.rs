use std::ops::Range;

struct Assignment {
    first: Range<u32>,
    second: Range<u32>,
}

impl Assignment {
    fn new(first: Range<u32>, second: Range<u32>) -> Self {
        Self { first, second }
    }
    fn full_overlap(&self) -> bool {
        self.first.full_overlap(&self.second)
    }

    fn any_overlap(&self) -> bool {
        self.first.has_overlap(&self.second)
    }
}

trait CheckOverlap {
    fn full_overlap(&self, other: &Range<u32>) -> bool;
    fn has_overlap(&self, other: &Range<u32>) -> bool;
}

impl CheckOverlap for Range<u32> {
    fn full_overlap(&self, other: &Range<u32>) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end)
    }

    fn has_overlap(&self, other: &Range<u32>) -> bool {
        self.start >= other.start && self.start <= other.end
            || self.end >= other.start && self.end <= other.end
            || other.start >= self.start && other.start <= self.end
            || other.end >= self.start && other.end <= self.end
    }
}

fn build_assignments(data: &[String]) -> Vec<Assignment> {
    data.iter().fold(Vec::new(), |mut acc, item| {
        let thing: Vec<u32> = item
            .split(&['-', ','])
            .map(|x| x.parse::<u32>().expect("invalid input"))
            .collect();

        let first = thing.first().expect("missing data!");
        let second = thing.get(1).expect("missing data!");
        let third = thing.get(2).expect("missing data!");
        let fourth = thing.get(3).expect("missing data!");
        let assignment = Assignment::new(*first..*second, *third..*fourth);
        acc.push(assignment);

        acc
    })
}

pub fn part1(data: &[String]) -> u32 {
    build_assignments(data)
        .iter()
        .fold(0, |mut acc, assignment| {
            if assignment.full_overlap() {
                acc += 1;
            }
            acc
        })
}

pub fn part2(data: &[String]) -> u32 {
    build_assignments(data)
        .iter()
        .fold(0, |mut acc, assignment| {
            if assignment.any_overlap() {
                acc += 1;
            }
            acc
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_full_overlap() {
        assert!(Assignment::new(0..5, 1..2).full_overlap());
        assert!(Assignment::new(0..5, 2..5).full_overlap());
        assert!(Assignment::new(0..5, 0..3).full_overlap());
        assert!(Assignment::new(3..4, 2..5).full_overlap());

        assert!(!Assignment::new(1..6, 3..7).full_overlap());
        assert!(!Assignment::new(2..7, 0..6).full_overlap());
    }

    #[test]
    fn test_any_overlap() {
        assert!(Assignment::new(0..5, 4..6).any_overlap());
        assert!(Assignment::new(0..5, 2..3).any_overlap());
        assert!(Assignment::new(1..5, 0..1).any_overlap());
        assert!(Assignment::new(3..4, 2..5).any_overlap());

        assert!(!Assignment::new(1..4, 5..7).any_overlap());
        assert!(!Assignment::new(2..7, 8..10).any_overlap());
    }

    #[test]
    fn test_day4_part1_sample() {
        let sample_data = read_data("day4/sample.txt");
        assert_eq!(part1(&sample_data), 2);
    }

    #[test]
    fn test_day4_part1() {
        let data = read_data("day4/full.txt");
        assert_eq!(part1(&data), 477);
    }

    #[test]
    fn test_day4_part2_sample() {
        let data = read_data("day4/sample.txt");
        assert_eq!(part2(&data), 4);
    }

    #[test]
    fn test_day4_part2() {
        let data = read_data("day4/full.txt");
        assert_eq!(part2(&data), 830);
    }
}