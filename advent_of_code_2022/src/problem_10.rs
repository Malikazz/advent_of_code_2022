use std::fs::read_to_string;

pub fn problem_10() {
    
    let data: String = read_to_string("src/assets/problem_10").unwrap();

    let mut commands: Vec<&str> = Vec::new();
    let mut signal_str: i32 = 0;

    let mut register_x = 0;
    let mut command_cycles: i32 = 0;
    let mut register_v: i32 = 1;
    let mut cycle_count = 0;

    let mut crt_array: Vec<Vec<String>> = Vec::new();

    for line in data.split("\n") {
        for part in line.split(" ") {
            commands.push(part);
            if part == "noop" {
                commands.push("0")
            }
        }
    }

    for command in commands.windows(2).step_by(2) {
        match command[0] {
            "noop" => {
                command_cycles = 1;
                register_x = command[1].parse().unwrap()
            }
            "addx" => {
                command_cycles = 2;
                register_x = command[1].parse().unwrap()
            }
            _ => {}
        }

        while command_cycles > 0 {
            cycle_count = cycle_count + 1;
            command_cycles = command_cycles - 1;

            draw_to_crt(cycle_count, register_v, &mut crt_array);

            signal_str = check_for_special_cycle(cycle_count, signal_str, register_v);
        }
        // do the add
        register_v = register_v + register_x;
    }

    for row in crt_array.iter() {
        print!("Problem 10: {:?}\n\n", row);
    }
}

fn create_crt_row() -> Vec<String> {
    return vec![
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
    ];
}

fn check_for_special_cycle(cycle_count: i32, mut signal_str: i32, register_v: i32) -> i32 {
    match cycle_count {
        20 => signal_str = signal_str + (register_v * 20),
        60 => signal_str = signal_str + (register_v * 60),
        100 => signal_str = signal_str + (register_v * 100),
        140 => signal_str = signal_str + (register_v * 140),
        180 => signal_str = signal_str + (register_v * 180),
        220 => signal_str = signal_str + (register_v * 220),
        _ => {}
    }
    signal_str
}
fn draw_to_crt(cycle_count: i32, register_v: i32, crt_array: &mut Vec<Vec<String>>) {
    let mut column: i32 = 0;
    let mut row: i32 = 0;

    if cycle_count <= 40 {
        if crt_array.len() == row as usize {
            crt_array.push(create_crt_row())
        }

        column = if (cycle_count % 40) == 0 {
            39
        } else {
            cycle_count % 40 - 1
        };

        if register_v == column || register_v + 1 == column || register_v - 1 == column {
            crt_array[row as usize][column as usize] = String::from("#");
            return;
        }

        crt_array[row as usize][column as usize] = String::from(".");
    } else {
        row = cycle_count / 40;
        column = if (cycle_count % 40) == 0 {
            39
        } else {
            cycle_count % 40 - 1
        };

        if crt_array.len() == row as usize {
            crt_array.push(create_crt_row())
        }

        if register_v == column || register_v + 1 == column || register_v - 1 == column {
            crt_array[row as usize][column as usize] = String::from("#");
            return;
        }
        crt_array[row as usize][column as usize] = String::from(".");
    }
}
