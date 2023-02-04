use std::fs::read_to_string;
use std::collections::HashMap;

pub fn problem_14() {
    let map = create_map();    

   // sand trys to fall left then right, can fall left or right multiple levels 


    print!("Problem 14: {:?}\n\n", map);
}


fn create_map() -> HashMap<i32,Vec<PointType>> {
    let mut data:Vec<Vec<Command>> = Vec::new();
    let mut map:HashMap<i32,Vec<PointType>> = HashMap::new();
    for line in read_to_string("src/assets/problem_14_test").unwrap().split('\n'){
        // place mark at location different between locations becomes a line
        // one number in the set will always be different
        // if its the first number we are moving left or right based on if its larger or smaller
        // if its the 2nd number we are moving up or down based on if its larger
        // the first number can be thought of as the col and the 2nd the row of that column
        // we always fill between numbers
        let mut temp: Vec<i32> = Vec::new();
        for commands in line.split(" -> ") {
            for command in commands.split(",") {
                temp.push(command.parse().unwrap())
            }
        }
        let mut command_vec: Vec<Command> = Vec::new();
        for command in temp.windows(2).step_by(2) {
            command_vec.push(Command{col: command[0], row: command[1]})
        }
        data.push(command_vec)
    }
    for commands in data.iter(){
        let mut is_same_line: bool = false;
        for command in commands.iter(){
            if !map.contains_key(&command.col) {
                // generate 200 values of air
                map.insert(command.col, (0..200).map(|x| PointType::Air).collect());
            }

            if is_same_line {
                
            } else {
                let temp_vec = map.get_mut(&command.col).unwrap();
                temp_vec[command.row as usize] = PointType::Rock;
            }
            is_same_line = true;
        }
    }
    // ensure no inbetween values are missing
    for items in map.keys().cloned().collect::<Vec<i32>>().windows(2) {
        if items[0] <= items[1] {
            for i in items[0]+1..items[1] {
                map.insert(i, (0..200).map(|x| PointType::Air).collect());
            }
        } 
    }

    map
}

#[derive(Debug)]
enum PointType {
    Air,
    Sand,
    Rock
}

struct Command {
    col: i32,
    row: i32
}