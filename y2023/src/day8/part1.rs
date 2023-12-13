use std::fs;

#[derive(Debug)]
struct position_data {
    position: String,
    left: String,
    right: String,
    left_pos: usize,
    right_pos: usize,
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File not read properly!");

    //Puts the command characters into a vector.
    let command_vec: Vec<char> = contents.lines().nth(0).unwrap().chars().collect();

    let mut position_vec: Vec<position_data> = Vec::new();
    //Stores each position and its data into a vector of position data.
    for current_line in contents.lines() {
        //If the current line is the first line or is useless, just keep going.
        if !current_line.contains("(") || current_line.is_empty() {
            continue;
        }

        //Stores the current position data in 
        let current_position: position_data = position_data { 
            position: current_line.split_whitespace().nth(0).unwrap().to_string(), 
            left: current_line.split(['(', ',', ')']).nth(1).unwrap().trim().to_string(), 
            right: current_line.split(['(', ',', ')']).nth(2).unwrap().trim().to_string(),
            left_pos: 0,
            right_pos: 0,
        };

        position_vec.push(current_position);
    }

    let mut starting_pos: usize = 0;
    //Assigns positions to left and right
    for i in 0..position_vec.len() {
        if position_vec[i].position == "AAA" {
            starting_pos = i;
        }
        //This finds the position of the left string in the vector
        let mut iterator: usize = 0;
        for leftposition in 0..position_vec.len() {
            //If the left's position is found, add it to the original's left_pos
            if position_vec[leftposition].position == position_vec[i].left {
                position_vec[i].left_pos = iterator;
                break;
            }

            iterator += 1;
        }

        //This finds the position of the right string in the vector
        let mut iterator: usize = 0;
        for rightposition in 0..position_vec.len() {
            //If the right's position is found, add it to the original's right_pos
            if position_vec[rightposition].position == position_vec[i].right {
                position_vec[i].right_pos = iterator;
                break;
            }

            iterator += 1;
        }
    }

    let mut steps:u128 = 0;

    let mut found: bool = false;
    let mut current_pos = starting_pos;
    while found == false {
        for index in 0..command_vec.len() {
            //Case of left
            if command_vec[index] == 'L' {
                current_pos = position_vec[current_pos].left_pos;
            }
            
            //Case of right
            if command_vec[index] == 'R' {
                current_pos = position_vec[current_pos].right_pos;
            }

            steps += 1;

            if position_vec[current_pos].position == "ZZZ" {
                found = true;
                break;
            }
        }
    }

    println!("Steps: {}", steps);
}