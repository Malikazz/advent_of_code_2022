
use std::fs::read_to_string;

pub fn problem_02() {
    let data: String = read_to_string("src/assets/problem_02").unwrap();
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
    print!("Problem 02: {:?}\n\n", points)
}
