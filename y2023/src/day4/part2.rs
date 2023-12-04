use std::{fs, any};

fn main() {
    //Value that holds the total sum
    let mut num_cards = 0;

    let mut num_card_type: Vec<u32> = Vec::new();
    let mut num_matches: Vec<u32> = Vec::new();

    let file_path = String::from("input.txt");

    println!("In file {file_path}");

    //Read the file into a string
    let contents = fs::read_to_string(file_path)
        .expect("File read properly!");

    for current in contents.lines() {
        num_cards += 1;
        num_card_type.push(1);
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
        
        let mut matched_ints = 0;
        //Then, searches the winning numbers for every number in the held vector
        for current_search in held_vec {
            //If the winning vector contains the current value, 
            if winning_vec.contains(&current_search) {
                matched_ints += 1;
            }
        }

        num_matches.push(matched_ints);
    }

    let mut j = 0;
    let len = num_card_type.len();
    for iterator in 0..len {
        j = iterator+1;

        while j <= iterator + num_matches[iterator] as usize {
            if j > len {
                break;
            }

            num_card_type[j] += num_card_type[iterator];

            j += 1;
        }
    }

    num_cards = 0;
    for num_current in num_card_type {
        num_cards += num_current;
    }

    println!("Sum of copies: {}", num_cards);
}