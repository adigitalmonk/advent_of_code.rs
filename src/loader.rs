use std::fs;

pub fn read_data(file_path: &str) -> Vec<String> {
    let appended_file_path = String::from("data/") + file_path;
    let file_data = fs::read_to_string(&appended_file_path)
        .unwrap_or_else(|_| panic!("couldn't load file {appended_file_path:?}"));

    let split_data = file_data.split('\n');
    let read_lines: Vec<String> = split_data.fold(Vec::new(), |mut acc, line| {
        acc.push(line.to_owned());
        acc
    });

    read_lines
}
