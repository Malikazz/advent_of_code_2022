use std::fs::read_to_string;
pub fn problem_12() {
    
    const UP:i32 = -1;
    const Down:i32 = 1;
    const Right:i32 = 1;
    const Left:i32 = -1;
    
    let start_label: u8 = "S".as_bytes()[0];
    let end_label: u8 = "E".as_bytes()[0];
    let mut start_point: Point = Point::new(0,0);
    let mut end_point: Point = Point::new(0,0);
    
    let data: String = read_to_string("src/assets/problem_12_test").unwrap();
    
    // u8 should allow for easy math in the path finding
    let mut mountain_map: Vec<Vec<u8>> = parse(&data);
    let mut closed_points: Vec<Point> = Vec::new();

    // find start and end
    for row in 0..mountain_map.len(){
        for col in 0..mountain_map[row].len(){
            if mountain_map[row][col] == start_label {
                start_point.col = col; start_point.row = row;
            }
            else if mountain_map[row][col] == end_label {
                end_point.col = col; end_point.row = row;
            }   
        }
    }

    print!("start:{:?} end:{:?}\n", start_point, end_point);

    // only step up one level, down as many as we want
    loop {

        break;
    }





    print!("Problem 12: {:?}\n\n", mountain_map)
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

fn find_point_value(point: Point, starting_point: Point, ending_point:Point) {
    
}

#[derive(Debug)]
struct Point{
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col:usize) -> Point{
        Point {row:row, col: col}
    }
}