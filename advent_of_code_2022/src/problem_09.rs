
use std::fs::read_to_string;

#[derive(Debug)]
struct CommandP9 {
    direction: String,
    count: usize,
}

pub fn problem_09() {
    let data: String = read_to_string("src/assets/problem_09").unwrap();
    let mut chars: Vec<String> = Vec::new();
    for line in data.split("\n") {
        for char in line.split(" ") {
            chars.push(String::from(char));
        }
    }
    print!("problem 09: {:?}\n\n", 0)
}
