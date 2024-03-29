use std::fs;

fn load_direct(file_path: &str) -> String {
    let appended_file_path = String::from("data/") + file_path;
    fs::read_to_string(&appended_file_path)
        .unwrap_or_else(|_| panic!("couldn't load file {appended_file_path:?}"))
}

pub fn read_data(file_path: &str) -> Vec<String> {
    let file_data = load_direct(file_path);
    let read_lines: Vec<String> = file_data.split('\n').map(ToOwned::to_owned).collect();

    read_lines
}

pub fn read_all_data(file_path: &str) -> String {
    load_direct(file_path)
}
