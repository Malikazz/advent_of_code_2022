use std::fs::File;
use std::io::prelude::*;

fn main() {
    //problem_01();
    problem_02()
}

fn problem_02(){
    let data: String = load_string("src/assets/paper_strat.txt");
    let mut plays: Vec<Vec<&str>> = Vec::new();

    let mut points: i32 = 0;

    for values in data.split("\n") {
        let temp = values;
        let mut tempVec: Vec<&str> = Vec::new(); 
        
        for item in temp.split(" ") {
            tempVec.push(item);
        }
        plays.push(tempVec);
    }
    for play in plays.iter(){
        // A X == Rock  == 1 
        // B Y == Papper == 2 
        // C Z == Sizzors == 3
        // x == loose
        // y draw
        // z win 
        // win 6 
        // draw 3
        // loss 0
        match play[0]{
            "A" => if play[1] == "X" { points = points + 3} else if play[1] == "Y" {points = points + 4} else {points = points + 8},
            "B" => if play[1] == "X" {points = points + 1} else if play[1] == "Y" {points = points + 5} else {points = points + 9},
            "C" => if play[1] == "X" {points = points + 2} else if play[1] == "Y" {points = points + 6} else  {points = points + 7}
            _ => print!("default")
        }
    }
    print!("{:?}\n", points)

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

    //print!("{:?}\n", elf_inventories[0].iter().sum::<i32>());
    //print!("{:?}\n", elf_inventories[1].iter().sum::<i32>());
    //print!("{:?}\n", elf_inventories[2].iter().sum::<i32>());
}

fn load_string(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    contents
}