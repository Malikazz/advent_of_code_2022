
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct FileNode {
    parent: usize,
    value: String,
    size: usize,
    children: HashSet<usize>,
}

pub fn problem_07() {
    let data: String = read_to_string("src/assets/problem_07").unwrap();
    let mut answer: usize = 0;
    let mut my_vec: Vec<FileNode> = Vec::new();

    let mut current_position: usize = 0;

    my_vec.push(FileNode {
        parent: usize::MAX as usize,
        size: 0,
        value: String::from("ROOT"),
        children: HashSet::new(),
    });

    for line in data.split("\n") {
        if line.contains("$ cd") {
            if line.contains("..") {
                current_position = my_vec[current_position].parent;
                continue;
            } else {
                my_vec.push(FileNode {
                    parent: current_position,
                    size: 0,
                    value: String::from(line),
                    children: HashSet::new(),
                });
                current_position = my_vec.len() - 1;
                continue;
            }
        }
        if line.contains("dir ") {
            continue;
        }
        if line == "$ ls" {
            continue;
        }
        // add child
        let mut temp_child: Vec<&str> = Vec::new();
        for child_part in line.split(" ") {
            temp_child.push(child_part);
        }
        my_vec.push(FileNode {
            parent: current_position,
            value: String::from(temp_child[1]),
            size: temp_child[0].parse::<usize>().unwrap(),
            children: HashSet::new(),
        });

        let current_len = my_vec.len();

        my_vec[current_position].children.insert(current_len - 1);
    }
    // fix children

    for item in 0..my_vec.len() {
        let mut parent = my_vec[item].parent;
        while parent != usize::MAX {
            let mut children = my_vec[item].children.clone();
            for inner_child in children {
                my_vec[parent].children.insert(inner_child);
            }
            parent = my_vec[parent].parent;
        }
    }

    for item in 0..my_vec.len() {
        if my_vec[item].children.len() > 0 {
            let mut temp_size: usize = 0;
            for child in my_vec[item].children.iter() {
                temp_size = temp_size + my_vec[*child].size;
            }
            my_vec[item].size = temp_size;
        }
    }
    let mut the_dir: &FileNode = &FileNode {
        parent: usize::MAX,
        value: String::from("fake"),
        size: usize::MAX,
        children: HashSet::new(),
    };
    for item in my_vec.iter() {
        if item.size >= 6_975_962 && item.size < the_dir.size {
            the_dir = item;
        }
    }
    // 23024038
    print!("Problems 07: {:?}\n\n", the_dir)
}
