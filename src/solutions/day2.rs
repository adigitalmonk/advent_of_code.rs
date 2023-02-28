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
        symbol
            .try_into()
            .unwrap_or_else(|_| panic!("bad input {symbol}"))
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

impl TryFrom<char> for Move {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(String::from("invalid")),
        }
    }
}

impl Game {
    const fn new(opponent: char, player: char) -> Self {
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

    const fn score(opponent: &Move, player: &Move) -> u32 {
        let choice_score = Self::score_choice(player);
        let match_score = Self::score_match(opponent, player);

        choice_score + match_score
    }

    const fn score_choice(player: &Move) -> u32 {
        match player {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    const fn score_match(opponent: &Move, player: &Move) -> u32 {
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
        let mut chars = game_input.chars();
        if let (Some(opponent), Some(' '), Some(player), None) =
            (chars.next(), chars.next(), chars.next(), chars.next())
        {
            let game = Game::new(opponent, player);
            acc.push(game);
        }

        acc
    })
}

pub fn part1(data: &[String]) -> u32 {
    build_games(data).iter().map(Game::run_for_win).sum()
}

pub fn part2(data: &[String]) -> u32 {
    build_games(data).iter().map(Game::run_for_outcome).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_part1_sample() {
        let sample_data = read_data("day2/sample.txt");
        assert_eq!(part1(&sample_data), 15);
    }

    #[test]
    fn test_part1() {
        let data = read_data("day2/full.txt");
        assert_eq!(part1(&data), 11150);
    }

    #[test]
    fn test_part2_sample() {
        let sample_data = read_data("day2/sample.txt");
        assert_eq!(part2(&sample_data), 12);
    }

    #[test]
    fn test_part2() {
        let data = read_data("day2/full.txt");
        assert_eq!(part2(&data), 8295);
    }
}
