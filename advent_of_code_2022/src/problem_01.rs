
use std::fs::read_to_string;

pub fn problem_01() {
    let data: String = read_to_string("src/assets/problem_01").unwrap();
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
    
    let inv_sum = elf_inventories[0].iter().sum::<i32>() + elf_inventories[1].iter().sum::<i32>() + elf_inventories[2].iter().sum::<i32>();
    
    print!("Problem 01: {:?}\n\n", inv_sum)
}
