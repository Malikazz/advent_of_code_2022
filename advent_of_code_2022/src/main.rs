use std::fs::File;
use std::io::prelude::*;

fn main() {
    problem_01()
}

fn problem_01(){
    let data: String = load_string("src/assets/elf_calories.txt");
    let mut elf_inventories: Vec<Vec<i32>> = Vec::new();
    
    let mut temp_vec: Vec<i32> = Vec::new();
    for item in data.split('\n') {
        if item == "" {
            elf_inventories.push(temp_vec);
            temp_vec = Vec::new();
            continue;
        }
        temp_vec.push(item.parse::<i32>().unwrap());
    }
    elf_inventories.sort_by(|a, b| b.iter().sum::<i32>().cmp(&a.iter().sum::<i32>()));

    print!("{:?}\n", elf_inventories[0].iter().sum::<i32>());
    print!("{:?}\n", elf_inventories[1].iter().sum::<i32>());
    print!("{:?}\n", elf_inventories[2].iter().sum::<i32>());
}

fn load_string(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    contents
}