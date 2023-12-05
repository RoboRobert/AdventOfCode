use std::{fs, any};

fn main() {
    let mut num_card_type: Vec<u32> = Vec::new();
    let mut num_matches: Vec<u32> = Vec::new();

    let file_path = String::from("example.txt");

    println!("In file {file_path}");

    //Read the file into a string
    let contents = fs::read_to_string(file_path)
        .expect("File not read properly!");

    for current in contents.lines() {
        num_card_type.push(1);

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
        for current_search in held_vec {
            //If the winning vector contains the current value, 
            if winning_vec.contains(&current_search) {
                matched_ints += 1;
            }
        }

        //Pushes the number of matches for each card type
        num_matches.push(matched_ints);
    }

    let mut j = 0;
    let len = num_card_type.len();
    //Iterates through the vector of the possessed number of each card type
    for iterator in 0..len {
        //j is used to iterate through a slice of num_card_type
        j = iterator+1;

        //Adds the number of cards to all the card types from 
        //1 below to the number of matches
        while j <= iterator + num_matches[iterator] as usize {
            if j > len {
                break;
            }

            num_card_type[j] += num_card_type[iterator];

            j += 1;
        }
    }

        //Value that holds the total sum
        let mut num_cards = 0;
    for num_current in num_card_type {
        num_cards += num_current;
    }

    println!("Sum of copies: {}", num_cards);
}