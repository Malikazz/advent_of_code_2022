use std::{fs::read_to_string, ops::RangeBounds};
extern crate yaml_rust;
use yaml_rust::YamlLoader;

struct Monkey {
    items: Vec<i32>,
    test: i32,
    operation: Operation,
    opertaion_value: i32,
    if_true: i32,
    if_false: i32,
}

impl Monkey {
    fn new(test: i32, if_true: i32, if_false: i32, operation: Operation, opertaion_value: i32, items:Vec<i32>) -> Monkey{
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

enum Operation{
    add,
    subtract,
    divide,
    multiply
}

pub fn problem_11() {
    let data = &read_to_string("src/assets/problem_11").unwrap();

    let mut monkeys: Vec<Monkey> = Vec::new();
    
    let mut temp_monkey = Monkey {
        items: Vec::new(),
                operation: Operation::add,
                opertaion_value: 0,
                test: 0,
                if_true: 0,
                if_false: 0,
    };

    for line in data.split("\n") {
        if line.contains("Monkey") {
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
            let line = line.replace(",", "");
            let line = line.replace("Starting items: ", "");
            for sub_line in line.split(" "){
                temp_monkey.items.push(sub_line.parse::<i32>().unwrap());
            }

        }
        else if line.contains("Operation") {
            let line = line.replace("Operation: new = old ", "");
            for sub_item in line.split(" "){
                match sub_item {
                    "+" => temp_monkey.operation = Operation::add,
                    "-" => temp_monkey.operation = Operation::subtract,
                    "*" => temp_monkey.operation = Operation::multiply,
                    "/" => temp_monkey.operation = Operation::divide,
                    _ => temp_monkey.opertaion_value = sub_item.parse().unwrap()
                }
            }
        }
        else if line.contains("Test") {
            let line = line.replace("Test: divisible by ", "");
            temp_monkey.test = line.parse().unwrap();
        }
        else if line.contains("If true") {
            let line = line.replace("If true: throw to monkey ", "");
            temp_monkey.if_true = line.parse().unwrap();
        }
        else if line.contains("If false") {
            let line = line.replace("If false: throw to monkey ", "");
            temp_monkey.if_false = line.parse().unwrap();
        }

    }

    print!("Problem 11: {:?}\n\n", monkeys)
}
