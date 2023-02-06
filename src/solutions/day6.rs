use std::collections::HashSet;

fn is_distinct(symbols: &[char]) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for item in symbols {
        set.insert(*item);
    }

    set.len() == symbols.len()
}

fn find_start_of_signal(input: &str, distinction: usize) -> usize {
    let symbols: Vec<char> = input.chars().collect();
    for i in 0..symbols.len() {
        if let Some(chunk) = symbols.get(i..i + distinction) {
            if is_distinct(chunk) {
                return i + distinction;
            }
        } else {
            return 0;
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
mod test {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_is_distinct() {
        assert!(is_distinct(&['a', 'b', 'c', 'd']));
        assert!(!is_distinct(&['a', 'a', 'c', 'd']));
    }

    #[test]
    fn test_day6_part1_samples() {
        assert_eq!(part1(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
        assert_eq!(part1(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(part1(&String::from("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(
            part1(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            10
        );
        assert_eq!(part1(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }

    #[test]
    fn test_day6_part1() {
        let data = read_all_data("day6/full.txt");
        assert_eq!(part1(&data), 1912);
    }

    #[test]
    fn test_day6_part2() {
        let data = read_all_data("day6/full.txt");
        assert_eq!(part2(&data), 2122);
    }
}
