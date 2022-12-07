use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;

fn main() {
    problem_07();
    problem_06();
    problem_05();
    //problem_02();
    //problem_03();
    //problem_04();
    //problem_01();
    
    
    
}

#[derive(Debug)]
struct FileNode {
    parent: usize,
    value: String,
    size: usize,
    children: Vec<usize>
}

fn problem_07(){
    let data: String = load_string("src/assets/problem_07");
    let mut answer: usize = 0;
    let mut my_vec: Vec<FileNode> = Vec::new();

    let mut current_position: usize = 0;

    for line in data.split("\n"){
        if line.contains("cd") {
            if line.contains(".."){
                current_position = my_vec[current_position].parent;
                continue;
            }else {
                let line = line.replace("$ cd", "");
                let line = line.trim();
                my_vec.push(FileNode{parent:current_position, size:0, value:String::from(line), children: Vec::new()});
                current_position = my_vec.len() -1;
                continue;
            }
            
        }
        if line.contains("dir") {
            my_vec.push(FileNode{parent:current_position, size:0, value:String::from(line), children: Vec::new()});
            continue;
        }
        if line == "$ ls" {
            continue;
        }
        // add child
        let mut temp_child: Vec<&str> = Vec::new();
        for child_part in line.split(" ") {
            temp_child.push(child_part);
        }
        my_vec.push(FileNode { parent: current_position, value:String::from(temp_child[1]), size: temp_child[0].parse::<usize>().unwrap(), children: Vec::new() });
        
        let current_len = my_vec.len();
        
        my_vec[current_position].children.push(current_len -1)
    }
    // fix children

    for item in 0..my_vec.len(){
        let parent = my_vec[item].parent;
        let mut children = my_vec[item].children.clone();
        my_vec[parent].children.append(&mut children);
    }
    

    for item in my_vec.iter(){
        if item.children.len() > 0 {
            let mut temp_size: usize = 0;
            for child in item.children.iter(){
                 temp_size = temp_size + my_vec[*child].size;
            }
            if temp_size < 100_001 {
                answer = answer + temp_size;
            }
        }
    }
    for item in my_vec.iter(){
        print!("{:?}\n", item)
    }
    print!("Problems 7 answer: {:?}\n", answer)
    
}

fn problem_06(){
    let data: String = load_string("src/assets/problem_06");
    let mut count: usize = 14;

    for item in data.as_bytes().to_vec().windows(14){
        let mut temp: HashSet<u8> = HashSet::new();
        temp.insert(item[0]);
        temp.insert(item[1]);
        temp.insert(item[2]);
        temp.insert(item[3]);
        temp.insert(item[4]);
        temp.insert(item[5]);
        temp.insert(item[6]);
        temp.insert(item[7]);
        temp.insert(item[8]);
        temp.insert(item[9]);
        temp.insert(item[10]);
        temp.insert(item[11]);
        temp.insert(item[12]);
        temp.insert(item[13]);
        
        
        if temp.len() == 14 {
            break;
        }
        count = count + 1;
    }

    print!("Problems 6 answer: {:?}\n", count)
}


#[derive(Debug)]
struct Problem5Command{
    move_count:usize,
    from:usize,
    too:usize
}


impl Problem5Command{
   pub fn new() -> Self{
        Self {move_count: 0, from: 0, too: 0}
    }
}
fn problem_05() {
    let data: String = load_string("src/assets/problem_05");
    let mut commands: Vec<Problem5Command> = Vec::new();
    let mut answer:String  = String::from("");
    // is a stack probably
    //[C]         [S] [H]                
    //[F] [B]     [C] [S]     [W]        
    //[B] [W]     [W] [M] [S] [B]        
    //[L] [H] [G] [L] [P] [F] [Q]        
    //[D] [P] [J] [F] [T] [G] [M] [T]    
    //[P] [G] [B] [N] [L] [W] [P] [W] [R]
    //[Z] [V] [W] [J] [J] [C] [T] [S] [C]
    //[S] [N] [F] [G] [W] [B] [H] [F] [N]
    // 1   2   3   4   5   6   7   8   9 

    let mut stacks: Vec<Vec<&str>> = vec![
        vec!["C","F","B","L","D","P","Z","S"],
        vec!["B","W","H","P","G","V","N"],
        vec!["G","J","B","W","F"],
        vec!["S","C","W","L","F","N","J","G"],
        vec!["H","S","M","P","T","L","J","W"],
        vec!["S","F","G","W","C","B"],
        vec!["W","B","Q","M","P","T","H"],
        vec!["T","W","S","F"],
        vec!["R","C","N"]];
    for stack in stacks.iter_mut(){
        stack.reverse();
    }
    for command in data.split("\n"){
        // remove all the text
        let mut numbers = command.replace("move","").replace("from", ",").replace("to",",").replace(" ", "");        
        let numbers_split = numbers.split(",").collect::<Vec<&str>>();

        let mut temp_command = Problem5Command::new();
        temp_command.move_count = numbers_split[0].parse::<usize>().unwrap();
        temp_command.from = numbers_split[1].parse::<usize>().unwrap() - 1 ;
        temp_command.too = numbers_split[2].parse::<usize>().unwrap() - 1 ;

        commands.push(temp_command);
    }

    for command in commands{
        // move using len and command numbers
        let mut to_move = stacks[command.from][(stacks[command.from].len() - command.move_count)..stacks[command.from].len()].to_vec();
        stacks[command.too].append(to_move.as_mut());

        for _i in 0..command.move_count{
            stacks[command.from].pop();
        }
        
    }

    for stack in stacks{
        answer.push_str(stack[stack.len()-1]);
    }
    print!("Problems 5 answer: {:?}\n", answer)
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
