use std::ops::RangeInclusive;

struct Assignment {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

impl Assignment {
    const fn new(first: RangeInclusive<u32>, second: RangeInclusive<u32>) -> Self {
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
    fn full_overlap(&self, other: &RangeInclusive<u32>) -> bool;
    fn has_overlap(&self, other: &RangeInclusive<u32>) -> bool;
}

impl CheckOverlap for RangeInclusive<u32> {
    fn full_overlap(&self, other: &RangeInclusive<u32>) -> bool {
        self.contains(other.start()) && self.contains(other.end())
            || other.contains(self.start()) && other.contains(self.end())
    }

    fn has_overlap(&self, other: &RangeInclusive<u32>) -> bool {
        self.contains(other.start())
            || self.contains(other.end())
            || other.contains(self.start())
            || other.contains(self.end())
    }
}

fn build_assignments(data: &[String]) -> Vec<Assignment> {
    data.iter()
        .map(|item| {
            item.split(&['-', ','])
                .map(|x| x.parse::<u32>().expect("invalid input"))
        })
        .map(|mut components| {
            if let (Some(first), Some(second), Some(third), Some(fourth), None) = (
                components.next(),
                components.next(),
                components.next(),
                components.next(),
                components.next(),
            ) {
                Assignment::new(first..=second, third..=fourth)
            } else {
                panic!("Bad input!")
            }
        })
        .collect()
}

pub fn part1(data: &[String]) -> u32 {
    build_assignments(data)
        .iter()
        .filter(|assignment| assignment.full_overlap())
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(data: &[String]) -> u32 {
    build_assignments(data)
        .iter()
        .filter(|assignment| assignment.any_overlap())
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_full_overlap() {
        let overlap_data = [
            (0..=5, 1..=2),
            (0..=5, 2..=5),
            (0..=5, 0..=3),
            (3..=4, 2..=5),
        ];
        for (left, right) in overlap_data {
            assert!(Assignment::new(left, right).full_overlap());
        }

        let no_overlap_data = [(1..=6, 3..=7), (2..=7, 0..=6)];
        for (left, right) in no_overlap_data {
            assert!(!Assignment::new(left, right).full_overlap());
        }
    }

    #[test]
    fn test_any_overlap() {
        let overlap_data = [
            (0..=5, 4..=6),
            (0..=5, 2..=3),
            (1..=5, 0..=1),
            (3..=4, 2..=5),
        ];
        for (left, right) in overlap_data {
            assert!(Assignment::new(left, right).any_overlap());
        }

        let no_overlap_data = [(1..=4, 5..=7), (2..=7, 8..=10)];
        for (left, right) in no_overlap_data {
            assert!(!Assignment::new(left, right).any_overlap());
        }
    }

    #[test]
    fn test_part1_sample() {
        let sample_data = read_data("day4/sample.txt");
        assert_eq!(part1(&sample_data), 2);
    }

    #[test]
    fn test_part1() {
        let data = read_data("day4/full.txt");
        assert_eq!(part1(&data), 477);
    }

    #[test]
    fn test_part2_sample() {
        let data = read_data("day4/sample.txt");
        assert_eq!(part2(&data), 4);
    }

    #[test]
    fn test_part2() {
        let data = read_data("day4/full.txt");
        assert_eq!(part2(&data), 830);
    }
}
