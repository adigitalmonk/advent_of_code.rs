fn build_team(data: &[String]) -> Vec<u32> {
    data.split(std::string::String::is_empty)
        .map(|backpack| {
            backpack
                .iter()
                .map(|item| item.parse::<u32>().unwrap_or_else(|_| panic!("bad input!")))
                .sum()
        })
        .collect()
}

pub fn part1(data: &[String]) -> u32 {
    let bags = build_team(data);
    bags.into_iter().max().unwrap_or(0)
}

pub fn part2(data: &[String]) -> u32 {
    let mut backpacks = build_team(data);
    backpacks.sort_unstable();
    backpacks.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_part1_sample_result() {
        let sample_data = read_data("day1/sample.txt");
        assert_eq!(part1(&sample_data), 24000);
    }

    #[test]
    fn test_part2_sample_result() {
        let sample_data = read_data("day1/sample.txt");
        assert_eq!(part2(&sample_data), 45000);
    }

    #[test]
    fn test_part1_result() {
        let data = read_data("day1/full.txt");
        assert_eq!(part1(&data), 70374);
    }

    #[test]
    fn test_part2_result() {
        let data = read_data("day1/full.txt");
        assert_eq!(part2(&data), 204_610);
    }
}
