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
        //Splits the lines into winning numbers and held numbers.
        let line_vec: Vec<&str> = current.split(['|']).collect();

        //This stores all the winning numbers in a u32 vector
        let winning_vec_str: Vec<&str> = line_vec[1].split([' ']).collect();
        let mut winning_vec: Vec<u32> = Vec::new();

        for winner in winning_vec_str {
            let current_winner = winner.parse::<i32>().unwrap_or(-1);
            if current_winner != -1 {
                winning_vec.push(current_winner as u32);
            }   
        }

        //This stores all the held numbers in a u32 vector
        let held_vec_str: Vec<&str> = line_vec[0].split([' ']).collect();
        let mut held_vec: Vec<u32> = Vec::new();
        for held in held_vec_str {
            let held_value = held.parse::<i32>().unwrap_or(-1);
            if held_value != -1 {
                held_vec.push(held_value as u32);
            }   
        }
        
        //Sorts the vectors for easy searching later
        winning_vec.sort();
        held_vec.sort();
        
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