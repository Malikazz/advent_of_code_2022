
use std::collections::HashMap;
use std::fs::read_to_string;
pub fn problem_03() {
    let data: String = read_to_string("src/assets/problem_03").unwrap();
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
    print!("Problem 03: {:?}\n\n", points)
}
