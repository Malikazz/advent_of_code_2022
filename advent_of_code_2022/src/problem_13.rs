use std::collections::HashMap;
use std::fs::read_to_string;
pub fn problem_13() {
    let data: String = read_to_string("src/assets/problem_13_test").unwrap();




    print!("Problem 13: {:?}\n\n", data)
}

fn parse(data: &String) -> Vec<Vec<i32>>{
    let mut temp: Vec<Vec<i32>> = Vec::new();
    let split_data: Vec<&str> = data.split("\n").collect();

    for pairs in split_data.windows(2).step_by(2){
        let mut vec_builder: Vec<i32> = Vec::new();
    }

    temp
}