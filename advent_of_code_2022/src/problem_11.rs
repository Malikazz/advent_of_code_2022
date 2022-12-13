use std::hash::{Hash, self};
use std::{fs::read_to_string, ops::RangeBounds};
use std::collections::HashMap;

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    test: i64,
    operation: Operation,
    opertaion_value: i64,
    if_true: i64,
    if_false: i64,
}

impl Monkey {
    fn new(test: i64, if_true: i64, if_false: i64, operation: Operation, opertaion_value: i64, items:Vec<i64>) -> Monkey{
        return Monkey { items: items, test: test, if_true: if_true, operation:operation, opertaion_value:opertaion_value, if_false:if_false }
    }

    fn operation(item: i32, operation: Operation, opertaion_value: i32) -> i32{
        match operation{
            Operation::add => return item + opertaion_value,
            Operation::subtract => return item - opertaion_value,
            Operation::multiply => return item * opertaion_value,
            Operation::divide => return item / opertaion_value,
            _ => panic!("you fucked up in operation on problem 11")
        }
    }
}
#[derive(Debug)]
enum Operation{
    add,
    subtract,
    divide,
    multiply,
    old
}

pub fn problem_11() {
    
    let data = &read_to_string("src/assets/problem_11").unwrap();
    let mut monkeys: Vec<Monkey> = parse_monkeys(data);
    let mut monkey_activity:Vec<i64> = seed_activity_vec(&monkeys);
    let common_divisor: i64 = get_common_divisor(&monkeys);
    
    for _i in 0..10_000{
        for monkey in 0..monkeys.len() {
            for item in 0..monkeys[monkey].items.len() {
                let temp_activity = monkey_activity[monkey];
                monkey_activity[monkey] = temp_activity + 1;

                match monkeys[monkey].operation {
                    Operation::add => monkeys[monkey].items[item] = monkeys[monkey].items[item] + monkeys[monkey].opertaion_value,
                    Operation::subtract => monkeys[monkey].items[item] = monkeys[monkey].items[item] - monkeys[monkey].opertaion_value,
                    Operation::multiply => monkeys[monkey].items[item] = monkeys[monkey].items[item] * monkeys[monkey].opertaion_value,
                    Operation::divide => monkeys[monkey].items[item] = monkeys[monkey].items[item] / monkeys[monkey].opertaion_value,
                    Operation::old => monkeys[monkey].items[item] = monkeys[monkey].items[item] * monkeys[monkey].items[item]
                }


                
                monkeys[monkey].items[item] = monkeys[monkey].items[item] % common_divisor;

                if monkeys[monkey].items[item] % monkeys[monkey].test == 0 {
                    let index = monkeys[monkey].if_true as usize;
                    let value = monkeys[monkey].items[item];
                    monkeys[index].items.push(value);

                } else {
                    let index = monkeys[monkey].if_false as usize;
                    let value = monkeys[monkey].items[item];
                    monkeys[index].items.push(value);
                }
            }
            // remove all items from this monkey
            for _i in 0..monkeys[monkey].items.len() {
                monkeys[monkey].items.pop();
            }
            
        }
    }
   
    monkey_activity.sort_by(|a, b| b.cmp(a));
    print!("Problem 11: {:?}\n\n", monkey_activity)
}

fn seed_activity_vec(monkey: &Vec<Monkey>) -> Vec<i64>{
    let mut vec: Vec<i64> = Vec::new();
    for _i in 0..monkey.len(){
        vec.push(0);
    }
    vec
}

fn parse_monkeys(data: &String) -> Vec<Monkey>{
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut first_run: bool = true;
    let mut temp_monkey = Monkey {
        items: Vec::new(),
                operation: Operation::add,
                opertaion_value: 0,
                test: 0,
                if_true: 0,
                if_false: 0,
    };

    for line in data.split("\n") {
        if line.contains("Monkey") && !first_run {
            monkeys.push(temp_monkey);
            temp_monkey = Monkey {
                items: Vec::new(),
                operation: Operation::add,
                opertaion_value: 0,
                test: 0,
                if_true: 0,
                if_false: 0,
            };
        } else if line == "" {
            continue;
        }
        if line.contains("Starting items") {
            let line = line.replace("Starting items: ", "");
            let line = line.replace(" ", "");
            for sub_line in line.split(","){
                temp_monkey.items.push(sub_line.parse::<i64>().unwrap());
            }

        }
        else if line.contains("Operation") {
            let line = line.replace("Operation: new = old ", "");
            let line = line.trim();
            for sub_item in line.split(" "){
                match sub_item {
                    "+" => temp_monkey.operation = Operation::add,
                    "-" => temp_monkey.operation = Operation::subtract,
                    "*" => temp_monkey.operation = Operation::multiply,
                    "/" => temp_monkey.operation = Operation::divide,
                    "old" => temp_monkey.operation = Operation::old,
                    _ => temp_monkey.opertaion_value = sub_item.parse().unwrap()
                }
            }
        }
        else if line.contains("Test") {
            let line = line.replace("Test: divisible by ", "");
            let line = line.trim();
            temp_monkey.test = line.parse().unwrap();
        }
        else if line.contains("If true") {
            let line = line.replace("If true: throw to monkey ", "");
            let line = line.trim();
            temp_monkey.if_true = line.parse().unwrap();
        }
        else if line.contains("If false") {
            let line = line.replace("If false: throw to monkey ", "");
            let line = line.trim();
            temp_monkey.if_false = line.parse().unwrap();
        }
        first_run = false;
    }
    monkeys.push(temp_monkey);
    monkeys
}

fn get_common_divisor(monkeys: &Vec<Monkey>) -> i64{
    let mut temp: i64 = 1;
    for m in monkeys.iter(){
        temp = temp * m.test;
    }
    temp
}