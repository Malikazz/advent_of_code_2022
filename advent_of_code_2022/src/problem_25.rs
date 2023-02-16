use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Lines;

pub fn problem_25() {
    let mut decimal_numbers: Vec<i64> = Vec::new();

    let lines = read_lines("src/assets/problem_25").unwrap();
    for line in lines {
        let snafu_line = line.unwrap();
        decimal_numbers.push(convert_sanfu_to_decimal(snafu_line.into_bytes()));
    }

    print!("Problem 25: {:?} \n \n", convert_decimal_to_snafu(decimal_numbers.iter().sum::<i64>()))
}

pub fn convert_sanfu_to_decimal(snafu_number: Vec<u8>) -> i64 {
    let mut current_position: i64 = 1;
    let mut dec_number: i64 = 0;
    // rev for left to right
    for item in snafu_number.iter().rev() {
        match item {
            b'2' => dec_number = dec_number + (current_position * 2),
            b'1' => dec_number = dec_number + (current_position * 1),
            b'0' => {}
            b'-' => dec_number = dec_number - (current_position * 1),
            b'=' => dec_number = dec_number - (current_position * 2),
            _ => {
                panic!("impossible SANFU value in match")
            }
        }
        current_position = current_position * 5;
    }
    dec_number
}

pub fn convert_decimal_to_snafu(number: i64) -> String {
    let mut remainder = number;
    let mut snafu_number = String::default();

    while remainder > 0 {
        let snafu_diget = match remainder % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!(),
        };
        snafu_number.push(snafu_diget);
        remainder += 2;
        remainder /= 5;
    }

    snafu_number = snafu_number.chars().rev().collect::<String>();
    snafu_number
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
