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

    let mut current_pos_vec: Vec<usize> = Vec::new();
    //Assigns positions to left and right
    for i in 0..position_vec.len() {
        if position_vec[i].position.ends_with("A") {
            current_pos_vec.push(i);
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

    let mut steps_vec: Vec<u128> = vec![0; current_pos_vec.len()];

    let mut steps:u128 = 0;

    let mut found: bool = false;
    while found == false {
        for command_index in 0..command_vec.len() {
            found = true;
            // dbg!(command_vec[command_index]);
            for current_index in 0..current_pos_vec.len() {
                //If any of them does not end in Z, then found is false
                if position_vec[current_pos_vec[current_index]].position.ends_with("Z") {
                    continue;
                }

                steps_vec[current_index] += 1;
                //Case of left
                if command_vec[command_index] == 'L' {
                    current_pos_vec[current_index] = position_vec[current_pos_vec[current_index]].left_pos;
                }
                
                //Case of right
                else {
                    current_pos_vec[current_index] = position_vec[current_pos_vec[current_index]].right_pos;
                }

                //If any of them does not end in Z, then found is false
                if !position_vec[current_pos_vec[current_index]].position.ends_with("Z") {
                    found = false;
                }
            }

            //If all of them end in Z, then it is over
            if found == true {
                break;
            }
        }
    }

    steps = findlcm(steps_vec);

    println!("Steps: {}", steps);
}

//I stole these functions from another thing that I was working on that required
//finding the least common multiple of an arbitrary collection of numbers lol
fn findlcm(numbers: Vec<u128>) -> u128 {
    let mut result = numbers[0];
    for number in numbers {
        result = lcm(result, number);
    }
    return result;
}

fn lcm(a: u128, b: u128) -> u128 {
    return a * (b / gcd(a, b));
}

// Function to return gcd of a and b 
fn gcd(a: u128, b: u128) -> u128 { 
    if a == 0 {
        return b; 
    }

    return gcd(b%a, a);
}