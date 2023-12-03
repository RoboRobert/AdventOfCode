use std::{fs, convert, thread::current};

fn main() {
    let mut digitFound = false;
    
    let mut total: u32 = 0;

    let mut digit1: u32 = 0;
    let mut digit2: u32 = 0;
    let mut current: u32 = 0;

    let file_path = String::from("input.txt");

    println!("In file {file_path}");

    //Read the file into a string
    let contents = fs::read_to_string(file_path)
        .expect("File read properly!");

    //This goes through the file one line at a time
    for word in contents.split_whitespace() {
        //Flag value that keeps track of whether or not the first digit has been found.
        digitFound = false;
        //This goes through every line one character at a time.
        for char in word.chars() {
            //If the character is a digit, do logic.
            if char.is_digit(10) {
                if digitFound == false {
                    digit1 = char.to_digit(10).unwrap();
                    digitFound = true;
                }

                //If the character is a digit, assume it is the last digit of the line.
                digit2 = char.to_digit(10).unwrap();
                
                print!("{}, ", char);
            }
        }
        //The current is a number made by putting digit1 in the 10s place and digit2 in the 1s place.
        current = (digit1*10) + digit2;
        total += current;
        println!("digit1: {digit1} digit2: {digit2} current: {current} sum: {total}");
    }

    println!("Total: {total}");
}