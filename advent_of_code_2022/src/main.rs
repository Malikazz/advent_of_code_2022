use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //problem_01();
    //problem_02();
    //problem_03()
    problem_04();
}

fn problem_04() {
    let data: String = load_string("src/assets/problem_04");
    let mut pairs: Vec<Vec<i32>> = Vec::new(); 

    let mut count: i32 = 0;
    // How often does a pair fully containt the other
    for line in data.split("\n"){
        for pair in line.split(","){
            let mut temp: Vec<i32> = Vec::new();
            for side in pair.split("-"){
                temp.push(side.parse::<i32>().unwrap())
            }
            pairs.push(temp);
        }
    }

    for elf_pairs in pairs.windows(2).step_by(2){
        let start1 = elf_pairs[0][0];
        let start2 = elf_pairs[1][0];
        let end1 = elf_pairs[0][1];
        let end2 = elf_pairs[1][1];

        if start1 > end2 || start2 > end1 {
            count = count + 1;
        }
    }
    print!("{:?}\n", ((pairs.len() / 2)  as i32) - count)

}
fn IsValueInPair(value: i32, pair: &Vec<i32>) -> bool{
    value >= pair[0] && value <= pair[1]
}

fn problem_03() {
    let data: String = load_string("src/assets/problem_03.txt");
    let mut ruck: Vec<&str> = Vec::new();
    let mut points: i32 = 0;
    let point_map: HashMap<String, i32> = HashMap::from([
        (String::from("a"), 1),
        (String::from("b"), 2),
        (String::from("c"), 3),
        (String::from("d"), 4),
        (String::from("e"), 5),
        (String::from("f"), 6),
        (String::from("g"), 7),
        (String::from("h"), 8),
        (String::from("i"), 9),
        (String::from("j"), 10),
        (String::from("k"), 11),
        (String::from("l"), 12),
        (String::from("m"), 13),
        (String::from("n"), 14),
        (String::from("o"), 15),
        (String::from("p"), 16),
        (String::from("q"), 17),
        (String::from("r"), 18),
        (String::from("s"), 19),
        (String::from("t"), 20),
        (String::from("u"), 21),
        (String::from("v"), 22),
        (String::from("w"), 23),
        (String::from("x"), 24),
        (String::from("y"), 25),
        (String::from("z"), 26),
        (String::from("A"), 27),
        (String::from("B"), 28),
        (String::from("C"), 29),
        (String::from("D"), 30),
        (String::from("E"), 31),
        (String::from("F"), 32),
        (String::from("G"), 33),
        (String::from("H"), 34),
        (String::from("I"), 35),
        (String::from("J"), 36),
        (String::from("K"), 37),
        (String::from("L"), 38),
        (String::from("M"), 39),
        (String::from("N"), 40),
        (String::from("O"), 41),
        (String::from("P"), 42),
        (String::from("Q"), 43),
        (String::from("R"), 44),
        (String::from("S"), 45),
        (String::from("T"), 46),
        (String::from("U"), 47),
        (String::from("V"), 48),
        (String::from("W"), 49),
        (String::from("X"), 50),
        (String::from("Y"), 51),
        (String::from("Z"), 52),
    ]);
    // all) items should only be in one side

    for line in data.split("\n") {
        ruck.push(line);
    }
    // find the duplicate item
    // stupid solution is just check everything
    for bags in ruck.windows(3).step_by(3) {
        for item in bags[0].chars(){
            if bags[1].contains(item) && bags[2].contains(item) {
                points = points + point_map[&String::from(item)];
                break;
            }
        }
    }
    print!("{:?}\n", points)
}

fn problem_02() {
    let data: String = load_string("src/assets/paper_strat.txt");
    let mut plays: Vec<Vec<&str>> = Vec::new();

    let mut points: i32 = 0;

    for values in data.split("\n") {
        let temp = values;
        let mut temp_vec: Vec<&str> = Vec::new();

        for item in temp.split(" ") {
            temp_vec.push(item);
        }
        plays.push(temp_vec);
    }
    // I assumed part two was gonna be hard but I probably dont need the extra loop
    for play in plays.iter() {
        // A X == Rock  == 1
        // B Y == Papper == 2
        // C Z == Sizzors == 3
        // x == loose
        // y draw
        // z win
        // win 6
        // draw 3
        // loss 0
        match play[0] {
            "A" => {
                if play[1] == "X" {
                    points = points + 3
                } else if play[1] == "Y" {
                    points = points + 4
                } else {
                    points = points + 8
                }
            }
            "B" => {
                if play[1] == "X" {
                    points = points + 1
                } else if play[1] == "Y" {
                    points = points + 5
                } else {
                    points = points + 9
                }
            }
            "C" => {
                if play[1] == "X" {
                    points = points + 2
                } else if play[1] == "Y" {
                    points = points + 6
                } else {
                    points = points + 7
                }
            }
            _ => print!("default"),
        }
    }
    print!("{:?}\n", points)
}

fn problem_01() {
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
