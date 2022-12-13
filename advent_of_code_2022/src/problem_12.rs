use std::fs::read_to_string;
pub fn problem_12() {
    
    let data: String = read_to_string("src/assets/problem_12_test").unwrap();
    
    // u8 should allow for easy math in the path finding
    let mut mountain_map: Vec<Vec<u8>> = parse(&data);

    // only step up one level, down as many as we want

    




    print!("Problem 12: {:?}\n\n", data)
}

fn parse(data: &String) -> Vec<Vec<u8>>{
    let mut mountain_map: Vec<Vec<u8>> = Vec::new();

    for line in data.split("\n") {
        let mut temp: Vec<u8> = Vec::new();
        for char in line.bytes() {
            temp.push(char);
        }
        mountain_map.push(temp);
    }
    mountain_map
}