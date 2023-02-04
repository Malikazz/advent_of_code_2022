use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn problem_25() {   
   // sand trys to fall left then right, can fall left or right multiple levels 
    if let Ok(lines) = read_lines("assets/problem_25_test"){
        for line in lines {
           if let Ok(snafu_line) = line {
            let snafu_vec = snafu_line.into_bytes();
            let mut current_position: i32 = 0;

            for item in snafu_vec.iter().rev(){
                match item {
                    b'2' => {},
                    b'1' => {},
                    b'0' => {},
                    b'-' => {},
                    b'=' => {},
                    _ => { panic!("impossible SANFU value in match")}
                }

                current_position = current_position + 1;
            }
           }
        } 
    }

    print!("Problem 25: \n\n");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}