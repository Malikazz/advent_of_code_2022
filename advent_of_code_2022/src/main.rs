use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::Command;
use std::process::CommandArgs;

fn main() {
    problem_05();
    problem_06();
    problem_07();
    print!("Problem 8 answer: {:?}\n", problem_08());
    problem_09();
    problem_10();
    //problem_02();
    //problem_03();
    //problem_04();
    //problem_01();
}

fn check_for_special_cycle(cycle_count: i32, mut signal_str: i32, register_v: i32) -> i32 {
    match cycle_count{
        20 => signal_str =  signal_str + (register_v * 20),
        60 => signal_str =  signal_str + (register_v * 60),
        100 => signal_str =  signal_str + (register_v * 100),
        140 => signal_str =  signal_str + (register_v * 140),
        180 => signal_str =  signal_str + (register_v * 180),
        220 => signal_str =  signal_str + (register_v * 220),
        _ => {}
    }
    signal_str
}
fn draw_to_crt(cycle_count: i32, register_v: i32, crt_array: &mut Vec<Vec<String>>) {
    let mut column: i32 = 0;
    let mut row: i32 = 0;

    if cycle_count <= 40 {
        column = cycle_count - 1;
        if register_v == cycle_count || register_v + 1 == cycle_count || register_v -1 == cycle_count {
            crt_array[row as usize][column as usize].push('#');
        }
        crt_array[row as usize][column as usize].push('.');
        return
    } else {
        column = (cycle_count % 40) -1;
        row = cycle_count / 40;
        if register_v == cycle_count || register_v + 1 == cycle_count || register_v -1 == cycle_count {
            crt_array[row as usize][column as usize].push('#');
        }
        crt_array[row as usize][column as usize].push('.');
    }
}

fn problem_10(){
    let data: String = load_string("src/assets/problem_10");
    let mut commands: Vec<&str> = Vec::new();
    let mut signal_str:i32 = 0;

    let mut register_x = 0;
    let mut command_cycles:i32 = 0;
    let mut register_v: i32 = 1;
    let mut cycle_count = 0;
    
    let mut crt_array: Vec<Vec<String>> = Vec::new();

    for line in data.split("\n") {
        for part in line.split(" ") {
            
            commands.push(part);
            if part == "noop" {
                commands.push("0")
            }
        }

    }

   for command in commands.windows(2).step_by(2){
    
    match command[0] {
        "noop" => {command_cycles = 1; register_x = command[1].parse().unwrap()},
        "addx" => {command_cycles = 2; register_x = command[1].parse().unwrap()},
        _ => {}
    }

    while command_cycles > 0 {
        cycle_count = cycle_count +1;
        command_cycles = command_cycles -1;
        
        draw_to_crt(cycle_count, register_v, &mut crt_array);
        
        signal_str = check_for_special_cycle(cycle_count, signal_str, register_v);
        
    }
    // do the add
    register_v = register_v + register_x;
   }

    


    print!("Problem 10: {:?}\n", signal_str);
}


#[derive(Debug)]
struct CommandP9 {
    direction: String,
    count: usize,
}

fn problem_09() {
    let data: String = load_string("src/assets/problem_09_test");
    let mut chars: Vec<String> = Vec::new();
    for line in data.split("\n") {
        for char in line.split(" ") {
            chars.push(String::from(char));
        }
    }
}

fn parse_p8() -> Vec<Vec<u32>> {
    let input = std::fs::read_to_string("src/assets/problem_08").unwrap();

    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn directions_p8(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();

    // Collect all the items from each row at the x offset
    let column = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();

    // split around our current point
    let (up, down) = column.split_at(y);
    let (left, right) = row.split_at(x);

    // reverse splits
    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    // normal splits
    let right = right[1..].to_vec();
    let down = down[1..].to_vec();

    [up, down, left, right]
}

pub fn problem_08() -> usize {
    // I kept having slighty off problems in this problem
    // so I went and found a solve to learn from.
    // credit here https://nickymeuleman.netlify.app/garden/aoc2022-day08
    let trees = parse_p8();
    let len = trees.len();

    // dont include the edges
    (1..len - 1)
        // get pairs to compare
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let height = trees[y][x];
            // split everything
            directions_p8(&trees, x, y)
                .iter()
                .map(|direction| {
                    direction
                        .iter()
                        .position(|h| *h >= height)
                        .map(|p| p + 1)
                        .unwrap_or_else(|| direction.len())
                })
                .product::<usize>()
        })
        .max()
        .unwrap()
}

#[derive(Debug)]
struct FileNode {
    parent: usize,
    value: String,
    size: usize,
    children: HashSet<usize>,
}

fn problem_07() {
    let data: String = load_string("src/assets/problem_07");
    let mut answer: usize = 0;
    let mut my_vec: Vec<FileNode> = Vec::new();

    let mut current_position: usize = 0;

    my_vec.push(FileNode {
        parent: usize::MAX as usize,
        size: 0,
        value: String::from("ROOT"),
        children: HashSet::new(),
    });

    for line in data.split("\n") {
        if line.contains("$ cd") {
            if line.contains("..") {
                current_position = my_vec[current_position].parent;
                continue;
            } else {
                my_vec.push(FileNode {
                    parent: current_position,
                    size: 0,
                    value: String::from(line),
                    children: HashSet::new(),
                });
                current_position = my_vec.len() - 1;
                continue;
            }
        }
        if line.contains("dir ") {
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
        my_vec.push(FileNode {
            parent: current_position,
            value: String::from(temp_child[1]),
            size: temp_child[0].parse::<usize>().unwrap(),
            children: HashSet::new(),
        });

        let current_len = my_vec.len();

        my_vec[current_position].children.insert(current_len - 1);
    }
    // fix children

    for item in 0..my_vec.len() {
        let mut parent = my_vec[item].parent;
        while parent != usize::MAX {
            let mut children = my_vec[item].children.clone();
            for inner_child in children {
                my_vec[parent].children.insert(inner_child);
            }
            parent = my_vec[parent].parent;
        }
    }

    for item in 0..my_vec.len() {
        if my_vec[item].children.len() > 0 {
            let mut temp_size: usize = 0;
            for child in my_vec[item].children.iter() {
                temp_size = temp_size + my_vec[*child].size;
            }
            my_vec[item].size = temp_size;
        }
    }
    let mut the_dir: &FileNode = &FileNode {
        parent: usize::MAX,
        value: String::from("fake"),
        size: usize::MAX,
        children: HashSet::new(),
    };
    for item in my_vec.iter() {
        if item.size >= 6_975_962 && item.size < the_dir.size {
            the_dir = item;
        }
    }
    // 23024038
    print!("Problems 7 answer: {:?}\n\n", the_dir)
}

fn problem_06() {
    let data: String = load_string("src/assets/problem_06");
    let mut count: usize = 14;

    for item in data.as_bytes().to_vec().windows(14) {
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

    print!("Problems 6 answer: {:?}\n\n", count)
}

#[derive(Debug)]
struct Problem5Command {
    move_count: usize,
    from: usize,
    too: usize,
}

impl Problem5Command {
    pub fn new() -> Self {
        Self {
            move_count: 0,
            from: 0,
            too: 0,
        }
    }
}
fn problem_05() {
    let data: String = load_string("src/assets/problem_05");
    let mut commands: Vec<Problem5Command> = Vec::new();
    let mut answer: String = String::from("");
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
        vec!["C", "F", "B", "L", "D", "P", "Z", "S"],
        vec!["B", "W", "H", "P", "G", "V", "N"],
        vec!["G", "J", "B", "W", "F"],
        vec!["S", "C", "W", "L", "F", "N", "J", "G"],
        vec!["H", "S", "M", "P", "T", "L", "J", "W"],
        vec!["S", "F", "G", "W", "C", "B"],
        vec!["W", "B", "Q", "M", "P", "T", "H"],
        vec!["T", "W", "S", "F"],
        vec!["R", "C", "N"],
    ];
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    for command in data.split("\n") {
        // remove all the text
        let mut numbers = command
            .replace("move", "")
            .replace("from", ",")
            .replace("to", ",")
            .replace(" ", "");
        let numbers_split = numbers.split(",").collect::<Vec<&str>>();

        let mut temp_command = Problem5Command::new();
        temp_command.move_count = numbers_split[0].parse::<usize>().unwrap();
        temp_command.from = numbers_split[1].parse::<usize>().unwrap() - 1;
        temp_command.too = numbers_split[2].parse::<usize>().unwrap() - 1;

        commands.push(temp_command);
    }

    for command in commands {
        // move using len and command numbers
        let mut to_move = stacks[command.from]
            [(stacks[command.from].len() - command.move_count)..stacks[command.from].len()]
            .to_vec();
        stacks[command.too].append(to_move.as_mut());

        for _i in 0..command.move_count {
            stacks[command.from].pop();
        }
    }

    for stack in stacks {
        answer.push_str(stack[stack.len() - 1]);
    }
    print!("Problems 5 answer: {:?}\n\n", answer)
}

fn problem_04() {
    let data: String = load_string("src/assets/problem_04");
    let mut pairs: Vec<Vec<i32>> = Vec::new();

    let mut count: i32 = 0;
    // How often does a pair fully containt the other
    for line in data.split("\n") {
        for pair in line.split(",") {
            let mut temp: Vec<i32> = Vec::new();
            for side in pair.split("-") {
                temp.push(side.parse::<i32>().unwrap())
            }
            pairs.push(temp);
        }
    }

    for elf_pairs in pairs.windows(2).step_by(2) {
        let start1 = elf_pairs[0][0];
        let start2 = elf_pairs[1][0];
        let end1 = elf_pairs[0][1];
        let end2 = elf_pairs[1][1];

        if start1 > end2 || start2 > end1 {
            count = count + 1;
        }
    }
    print!("{:?}\n", ((pairs.len() / 2) as i32) - count)
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
        for item in bags[0].chars() {
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
