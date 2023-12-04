use std::{fs, any};

fn main() {
    //Value that holds the total sum
    let mut sum = 0;

    let file_path = String::from("input.txt");

    println!("In file {file_path}");

    //Read the file into a string
    let contents = fs::read_to_string(file_path)
        .expect("File read properly!");

    for current in contents.lines() {
        //This stores all the winning numbers in a u32 vector
        let winning_vec_str: Vec<&str> =  current.split("|").collect::<Vec<&str>>()[1].split_whitespace().collect();
        let mut winning_vec: Vec<u32> = Vec::new();
        for winner in winning_vec_str {
            winning_vec.push(winner.parse::<u32>().unwrap());
                
        }

        //This stores all the held numbers in a u32 vector
        let held_vec_str: Vec<&str> = current.split("|").collect::<Vec<&str>>()[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().collect();
        let mut held_vec: Vec<u32> = Vec::new();
        for held in held_vec_str {
            held_vec.push(held.parse::<u32>().unwrap());
        }

        let mut matched_ints = 0;
        //Then, searches the winning numbers for every number in the held vector
        for current_search in &held_vec {
            //If the winning vector contains the current value, 
            if winning_vec.contains(&current_search) {
                matched_ints += 1;
            }
        }

        let mut worth = 0;
        //Then, searches the winning numbers for every number in the held vector
        for current_search in held_vec {
            //If the winning vector contains the current value, 
            if winning_vec.contains(&current_search) {
                if worth == 0 {
                    worth = 1;
                }
                else {
                    worth *= 2;
                }
            }
        }

        sum += worth;
    }

    println!("Sum of held winners: {}", sum);
}