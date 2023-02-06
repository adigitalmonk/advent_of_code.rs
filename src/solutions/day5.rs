use std::{collections::VecDeque, string::String};

#[derive(Debug)]
struct Ship {
    columns: Vec<Column>,
    indices: Vec<usize>,
}

impl Ship {
    fn new(length: usize) -> Self {
        let mut indices: Vec<usize> = Vec::new();
        for i in (0..length - 2).step_by(4) {
            indices.push(i + 1);
        }
        let mut columns: Vec<Column> = Vec::with_capacity(length);
        for _ in 0..indices.len() {
            columns.push(Column::new());
        }

        Self { columns, indices }
    }

    fn apply(&mut self, action: &Move) {
        let source_column = &mut self.columns[action.from - 1];

        let mut moved_items: Vec<char> = Vec::new();
        for _ in 0..action.stacks {
            let pop_result = source_column.remove();
            if let Some(item) = pop_result {
                moved_items.push(item);
            }
        }

        let target_column = &mut self.columns[action.to - 1];
        for item in moved_items {
            target_column.stack(item);
        }
    }

    fn apply_stable(&mut self, action: &Move) {
        let source_column = &mut self.columns[action.from - 1];

        let mut moved_items: Vec<char> = Vec::new();
        for _ in 0..action.stacks {
            let pop_result = source_column.remove();
            if let Some(item) = pop_result {
                moved_items.push(item);
            }
        }

        let target_column = &mut self.columns[action.to - 1];
        moved_items.reverse();
        for item in moved_items {
            target_column.stack(item);
        }
    }

    fn get_message(&self) -> String {
        let mut letters = String::new();
        for column in &self.columns {
            if let Some(letter) = column.top() {
                letters.push(*letter);
            }
        }
        letters
    }

    fn add(&mut self, raw: &str) {
        let raw_columns = self.string_to_columns(raw);
        for (col_idx, column_item) in raw_columns {
            self.add_to_column(col_idx, column_item);
        }
    }

    fn add_to_column(&mut self, idx: usize, item: char) {
        let this_column = &mut self.columns[idx];
        this_column.add(item);
    }

    fn string_to_columns(&self, raw: &str) -> Vec<(usize, char)> {
        let mut raw_columns: Vec<(usize, char)> = Vec::new();
        for (idx, raw_index) in self.indices.iter().enumerate() {
            let index_in_raw = *raw_index;
            let col_at_index = raw
                .get(index_in_raw..=index_in_raw)
                .expect("Invalid input")
                .chars()
                .next()
                .expect("More invalid input!");
            if col_at_index != ' ' {
                raw_columns.push((idx, col_at_index));
            }
        }

        raw_columns
    }
}

#[derive(Debug)]
struct Column {
    items: VecDeque<char>,
}

impl Column {
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    fn add(&mut self, item: char) {
        self.items.push_back(item);
    }

    fn stack(&mut self, item: char) {
        self.items.push_front(item);
    }

    fn remove(&mut self) -> Option<char> {
        self.items.pop_front()
    }

    fn top(&self) -> Option<&char> {
        self.items.front()
    }
}

fn build_ship(data: &[String]) -> Ship {
    let ship = Ship::new(data[0].len());
    data.iter().fold(ship, |mut ship_now, row| {
        ship_now.add(row);
        ship_now
    })
}

#[derive(Debug)]
struct Move {
    stacks: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from(raw: &str) -> Self {
        let parsed: Vec<&str> = raw.split(' ').collect();
        let stacks = string_to_usize(parsed[1]);
        let from = string_to_usize(parsed[3]);
        let to = string_to_usize(parsed[5]);

        Self { stacks, from, to }
    }
}

fn parse_instructions(data: &[String]) -> Vec<Move> {
    data.iter().fold(Vec::new(), |mut acc, raw_move| {
        acc.push(Move::from(raw_move));
        acc
    })
}

fn string_to_usize(original: &str) -> usize {
    let base = original.parse::<u16>().expect("invalid instruction input");
    usize::try_from(base).expect("invalid instruction input")
}

fn parse_input(full_data: &[String]) -> (Ship, Vec<Move>) {
    let split_at = full_data
        .iter()
        .position(String::is_empty)
        .expect("missing separator!");
    let ship_data = full_data.get(0..split_at - 1).expect("bad boat data pull");
    let instr_data = full_data.get(split_at + 1..).expect("bad instr data");

    (build_ship(ship_data), parse_instructions(instr_data))
}

pub fn part1(full_data: &[String]) -> String {
    let (mut ship, instructions) = parse_input(full_data);

    for instruction in &instructions {
        ship.apply(instruction);
    }

    ship.get_message()
}

pub fn part2(full_data: &[String]) -> String {
    let (mut ship, instructions) = parse_input(full_data);

    for instruction in &instructions {
        ship.apply_stable(instruction);
    }

    ship.get_message()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_part1_sample() {
        let sample_data = read_data("day5/sample.txt");
        assert_eq!(part1(&sample_data), "CMZ");
    }

    #[test]
    fn test_part1() {
        let data = read_data("day5/full.txt");
        assert_eq!(part1(&data), "QGTHFZBHV");
    }

    #[test]
    fn test_part2_sample() {
        let sample_data = read_data("day5/sample.txt");
        assert_eq!(part2(&sample_data), "MCD");
    }

    #[test]
    fn test_part2() {
        let data = read_data("day5/full.txt");
        assert_eq!(part2(&data), "MGDMPSZTM");
    }
}
