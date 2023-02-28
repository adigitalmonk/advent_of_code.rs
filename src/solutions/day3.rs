use std::collections::HashSet;

struct Rucksack {
    everything: HashSet<char>,
    left_pocket: HashSet<char>,
    right_pocket: HashSet<char>,
}

trait Prioritizable {
    fn priority(self) -> u32;
}

impl Prioritizable for char {
    fn priority(self) -> u32 {
        let value = self as u32;
        if (97..=122).contains(&value) {
            value - 96
        } else if (65..=90).contains(&value) {
            value - 38
        } else {
            panic!("invalid priority input")
        }
    }
}

impl Rucksack {
    fn new(raw: &String) -> Self {
        let pack_length = raw.len();
        let (left_raw, right_raw) = raw.split_at(pack_length / 2);

        Self {
            everything: raw.chars().collect(),
            left_pocket: left_raw.chars().collect(),
            right_pocket: right_raw.chars().collect(),
        }
    }

    fn get_badge_priority(&self) -> u32 {
        let badge = self
            .left_pocket
            .intersection(&self.right_pocket)
            .next()
            .expect("invalid pocket contents");

        (*badge).priority()
    }

    fn get_shared_badge_priority(sack_a: &Self, sack_b: &Self, sack_c: &Self) -> u32 {
        let shared_badge = sack_a
            .everything
            .iter()
            .filter(|k| sack_b.everything.contains(*k))
            .find(|k| sack_c.everything.contains(*k))
            .expect("invalid bag contents");

        (*shared_badge).priority()
    }
}

fn build_packs(data: &[String]) -> Vec<Rucksack> {
    data.iter().map(Rucksack::new).collect()
}

pub fn part1(data: &[String]) -> u32 {
    build_packs(data)
        .iter()
        .map(Rucksack::get_badge_priority)
        .sum()
}

pub fn part2(data: &[String]) -> u32 {
    build_packs(data)
        .chunks(3)
        .map(|sacks| Rucksack::get_shared_badge_priority(&sacks[0], &sacks[1], &sacks[2]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_priority_values() {
        for (letter, expected) in [('a', 1), ('z', 26), ('A', 27), ('Z', 52)] {
            assert_eq!(letter.priority(), expected);
        }
    }

    #[test]
    fn test_part1_sample() {
        let sample_data = read_data("day3/sample.txt");
        assert_eq!(part1(&sample_data), 157);
    }

    #[test]
    fn test_part1() {
        let data = read_data("day3/full.txt");
        assert_eq!(part1(&data), 8202);
    }

    #[test]
    fn test_part2_sample() {
        let sample_data = read_data("day3/sample.txt");
        assert_eq!(part2(&sample_data), 70);
    }

    #[test]
    fn test_part2() {
        let data = read_data("day3/full.txt");
        assert_eq!(part2(&data), 2864);
    }
}
