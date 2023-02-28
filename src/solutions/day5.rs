use std::{collections::VecDeque, str::FromStr, string::String};

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

    fn apply(&mut self, action: &Instruction) {
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

    fn apply_stable(&mut self, action: &Instruction) {
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
        self.columns
            .iter()
            .map(|column| column.top().unwrap())
            .collect()
    }

    fn add(&mut self, raw: &str) {
        for (col_idx, column_item) in self.string_to_columns(raw) {
            self.add_to_column(col_idx, column_item);
        }
    }

    fn add_to_column(&mut self, idx: usize, item: char) {
        self.columns[idx].add(item);
    }

    fn string_to_columns(&self, raw: &str) -> Vec<(usize, char)> {
        self.indices
            .iter()
            .enumerate()
            .map(|(idx, raw_index)| {
                let raw_index = *raw_index;
                let col_at_index = raw.chars().nth(raw_index).unwrap();
                (idx, col_at_index)
            })
            .filter(|(_, col)| *col != ' ')
            .collect()
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
    let mut ship = Ship::new(data[0].len());

    for row in data {
        ship.add(row);
    }

    ship
}

#[derive(Debug)]
struct Instruction {
    stacks: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed = s.split(' ');
        parsed.next();
        if let (Some(stacks_in), _, Some(from_in), _, Some(to_in)) = (
            parsed.next(),
            parsed.next(),
            parsed.next(),
            parsed.next(),
            parsed.next(),
        ) {
            Ok(Self {
                stacks: string_to_usize(stacks_in),
                from: string_to_usize(from_in),
                to: string_to_usize(to_in),
            })
        } else {
            Err(String::from("ohno"))
        }
    }
}

fn parse_instructions(data: &[String]) -> Vec<Instruction> {
    data.iter()
        .map(|instr| instr.as_str().parse::<Instruction>().unwrap())
        .collect()
}

fn string_to_usize(original: &str) -> usize {
    let base = original.parse::<u16>().expect("invalid instruction input");
    usize::try_from(base).expect("invalid instruction input")
}

fn parse_input(full_data: &[String]) -> (Ship, Vec<Instruction>) {
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
