
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn problem_06() {
    let data: String = read_to_string("src/assets/problem_06").unwrap();
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

    print!("Problems 06: {:?}\n\n", count)
}
