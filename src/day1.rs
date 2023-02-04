fn build_team(data: &Vec<String>) -> Vec<u32> {
    let mut team: Vec<u32> = Vec::new();
    let mut this_backpack = 0;

    for calories in data {
        if calories.is_empty() {
            team.push(this_backpack);
            this_backpack = 0;
        } else {
            let calorie_count = calories.parse::<u32>().expect("Not a number");
            this_backpack += calorie_count;
        }
    }

    team
}

pub fn part1(data: &Vec<String>) -> u32 {
    let bags = build_team(data);
    bags.iter().fold(0, |max, current| {
        if current < &max {
            max
        } else {
            current.to_owned()
        }
    })
}

pub fn part2(data: &Vec<String>) -> u32 {
    let mut backpacks = build_team(data);
    backpacks.sort_unstable();
    backpacks.iter().rev().take(3).fold(0, |mut acc, backpack| {
        acc += backpack;
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample_result() {
        let day1_sample_data = crate::loader::read_data("day1/sample.txt");
        assert_eq!(part1(&day1_sample_data), 24000);
    }

    #[test]
    fn test_part2_sample_result() {
        let day1_sample_data = crate::loader::read_data("day1/sample.txt");
        assert_eq!(part2(&day1_sample_data), 45000);
    }

    #[test]
    fn test_part1_result() {
        let day1_data = crate::loader::read_data("day1/full.txt");
        assert_eq!(part1(&day1_data), 70374);
    }

    #[test]
    fn test_part2_result() {
        let day2_data = crate::loader::read_data("day1/full.txt");
        assert_eq!(part2(&day2_data), 204610);
    }
}
