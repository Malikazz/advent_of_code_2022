
use std::fs::read_to_string;

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
pub fn problem_05() {
    let data: String = read_to_string("src/assets/problem_05").unwrap();
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
    print!("Problems 05: {:?}\n\n", answer)
}
