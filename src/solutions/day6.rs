use std::collections::HashSet;

fn is_distinct(symbols: &[char]) -> bool {
    let set: HashSet<char> = symbols.iter().copied().collect();
    set.len() == symbols.len()
}

fn find_start_of_signal(input: &str, distinction: usize) -> usize {
    let symbols: Vec<char> = input.chars().collect();
    for i in 0..symbols.len() {
        if let Some(chunk) = symbols.get(i..i + distinction) {
            if is_distinct(chunk) {
                return i + distinction;
            }
        }
    }
    0
}

pub fn part1(raw: &str) -> usize {
    find_start_of_signal(raw, 4)
}

pub fn part2(raw: &str) -> usize {
    find_start_of_signal(raw, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_is_distinct() {
        assert!(is_distinct(&['a', 'b', 'c', 'd']));
        assert!(!is_distinct(&['a', 'a', 'c', 'd']));
    }

    #[test]
    fn test_part1_samples() {
        let test_data = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (test_string, expected) in test_data {
            assert_eq!(part1(&String::from(test_string)), expected);
        }
    }

    #[test]
    fn test_part1() {
        let data = read_all_data("day6/full.txt");
        assert_eq!(part1(&data), 1912);
    }

    #[test]
    fn test_part2() {
        let data = read_all_data("day6/full.txt");
        assert_eq!(part2(&data), 2122);
    }
}
