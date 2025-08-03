use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::process;

pub fn extract_json(file_paths: &[String]) -> Vec<Value> {
    file_paths
        .iter()
        .map(|file_path| {
            let file = File::open(file_path).unwrap_or_else(|err| {
                eprintln!("Error opening file: '{}': {}", file_path, err);
                process::exit(1);
            });

            let rdr = BufReader::new(file);
            serde_json::from_reader(rdr).unwrap_or_else(|err| {
                eprintln!("Failed to parse as JSON '{}': {}", file_path, err);
                process::exit(1);
            })
        })
        .collect()
}

pub fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_owned()
}
