struct Game {
    opponent: char,
    player: char,
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn for_win(symbol: char) -> Self {
        match symbol {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Invalid move input"),
        }
    }

    fn for_outcome(opponent: &Self, outcome: char) -> Self {
        match (opponent, outcome) {
            (Self::Rock, 'Y') | (Self::Paper, 'X') | (Self::Scissors, 'Z') => Self::Rock,
            (Self::Rock, 'X') | (Self::Paper, 'Z') | (Self::Scissors, 'Y') => Self::Scissors,
            (Self::Rock, 'Z') | (Self::Paper, 'Y') | (Self::Scissors, 'X') => Self::Paper,
            _ => panic!("Invalid move input"),
        }
    }
}

impl Game {
    fn new(opponent: char, player: char) -> Self {
        Self { opponent, player }
    }

    fn run_for_win(&self) -> u32 {
        let player = Move::for_win(self.player);
        let opponent = Move::for_win(self.opponent);
        Self::score(&opponent, &player)
    }

    fn run_for_outcome(&self) -> u32 {
        let opponent = Move::for_win(self.opponent);
        let player = Move::for_outcome(&opponent, self.player);

        Self::score(&opponent, &player)
    }

    fn score(opponent: &Move, player: &Move) -> u32 {
        let choice_score = Self::score_choice(player);
        let match_score = Self::score_match(opponent, player);

        choice_score + match_score
    }

    fn score_choice(player: &Move) -> u32 {
        match player {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn score_match(opponent: &Move, player: &Move) -> u32 {
        match (opponent, player) {
            (Move::Rock, Move::Rock)
            | (Move::Scissors, Move::Scissors)
            | (Move::Paper, Move::Paper) => 3,
            (Move::Rock, Move::Scissors)
            | (Move::Scissors, Move::Paper)
            | (Move::Paper, Move::Rock) => 0,
            (Move::Rock, Move::Paper)
            | (Move::Scissors, Move::Rock)
            | (Move::Paper, Move::Scissors) => 6,
        }
    }
}

fn build_games(data: &[String]) -> Vec<Game> {
    data.iter().fold(Vec::new(), |mut acc, game_input| {
        let game_raw: Vec<&str> = game_input.split(' ').collect();

        let opponent = game_raw[0].chars().next().expect("wasn't a letter");
        let player = game_raw[1].chars().next().expect("wasn't a letter");
        let game = Game::new(opponent, player);
        acc.push(game);

        acc
    })
}

pub fn part1(data: &[String]) -> u32 {
    build_games(data).iter().fold(0, |mut acc, game| {
        acc += game.run_for_win();
        acc
    })
}

pub fn part2(data: &[String]) -> u32 {
    build_games(data).iter().fold(0, |mut acc, game| {
        acc += game.run_for_outcome();
        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_part1_sample() {
        let sample_data = read_data("day2/sample.txt");
        assert_eq!(part1(&sample_data), 15)
    }

    #[test]
    fn test_part1() {
        let data = read_data("day2/full.txt");
        assert_eq!(part1(&data), 11150)
    }

    #[test]
    fn test_part2_sample() {
        let sample_data = read_data("day2/sample.txt");
        assert_eq!(part2(&sample_data), 12)
    }

    #[test]
    fn test_part2() {
        let data = read_data("day2/full.txt");
        assert_eq!(part2(&data), 8295)
    }
}
