use std::collections::HashSet;

struct Rucksack {
    everything: HashSet<char>,
    left_pocket: HashSet<char>,
    right_pocket: HashSet<char>,
}

impl Rucksack {
    fn new(raw: &String) -> Self {
        let pack_length = raw.len();
        let (left_raw, right_raw) = raw.split_at(pack_length / 2);
        let left_pocket = left_raw.chars().collect::<HashSet<char>>();
        let right_pocket = right_raw.chars().collect::<HashSet<char>>();
        let everything = raw.chars().collect::<HashSet<char>>();

        Self {
            everything,
            left_pocket,
            right_pocket,
        }
    }

    fn get_badge_priority(&self) -> u32 {
        let badge = self
            .left_pocket
            .intersection(&self.right_pocket)
            .next()
            .expect("invalid pocket contents");

        get_priority(*badge)
    }

    fn get_shared_badge_priority(sack_a: &Self, sack_b: &Self, sack_c: &Self) -> u32 {
        let common_badge = sack_a
            .everything
            .iter()
            .filter(|k| sack_b.everything.contains(*k))
            .find(|k| sack_c.everything.contains(*k))
            .expect("invalid bag contents");

        get_priority(*common_badge)
    }
}

fn get_priority(symbol: char) -> u32 {
    let value = symbol as u32;
    if (97..=122).contains(&value) {
        value - 96
    } else if (65..=90).contains(&value) {
        value - 38 // 65 - 27
    } else {
        panic!("invalid priority input")
    }
}

fn build_packs(data: &[String]) -> Vec<Rucksack> {
    data.iter().fold(Vec::new(), |mut acc, pack_raw| {
        let pack = Rucksack::new(pack_raw);
        acc.push(pack);
        acc
    })
}

pub fn part1(data: &[String]) -> u32 {
    build_packs(data).iter().fold(0, |mut acc, sack| {
        acc += sack.get_badge_priority();
        acc
    })
}

pub fn part2(data: &[String]) -> u32 {
    build_packs(data).chunks(3).fold(0, |mut acc, sacks| {
        let sack_a = &sacks[0];
        let sack_b = &sacks[1];
        let sack_c = &sacks[2];

        acc += Rucksack::get_shared_badge_priority(sack_a, sack_b, sack_c);

        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_priority_values() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
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
