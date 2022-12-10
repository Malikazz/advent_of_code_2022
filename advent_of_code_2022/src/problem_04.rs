
use std::fs::read_to_string;

pub fn problem_04() {
    let data: String = read_to_string("src/assets/problem_04").unwrap();
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
    print!("Problem 04: {:?}\n\n", ((pairs.len() / 2) as i32) - count)
}
