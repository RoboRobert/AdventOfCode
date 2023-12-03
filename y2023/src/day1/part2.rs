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
        let converted = convert_string(word);

        //Flag value that keeps track of whether or not the first digit has been found.
        digitFound = false;
        //This goes through every line one character at a time.
        for char in converted.chars() {
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

// This puts the proper digits inside the corresponding words
fn convert_string(line: &str) -> String {
    let mut word_found = true;
    let mut converted_string = String::from(line);

    let mut current_index = 0;

    if converted_string.contains("one") {
        current_index = converted_string.find("one").unwrap() + 1;
        converted_string.insert(current_index, '1');

        if converted_string.contains("one") {
            current_index = converted_string.rfind("one").unwrap() + 1;
            converted_string.insert(current_index, '1');
        }
        word_found = true;
    }
    
    if converted_string.contains("two") {
        current_index = converted_string.find("two").unwrap() + 1;
        converted_string.insert(current_index, '2');

        if converted_string.contains("two") {
            current_index = converted_string.rfind("two").unwrap() + 1;
            converted_string.insert(current_index, '2');
        }
        word_found = true;
    }
    
    if converted_string.contains("three") {
        current_index = converted_string.find("three").unwrap() + 1;
        converted_string.insert(current_index, '3');

        if converted_string.contains("three") {
            current_index = converted_string.rfind("three").unwrap() + 1;
            converted_string.insert(current_index, '3');
        }

        word_found = true;
    }

    if converted_string.contains("four") {
        current_index = converted_string.find("four").unwrap() + 1;
        converted_string.insert(current_index, '4');

        if converted_string.contains("four") {
            current_index = converted_string.rfind("four").unwrap() + 1;
            converted_string.insert(current_index, '4');
        }
        word_found = true;
    }

    if converted_string.contains("five") {
        current_index = converted_string.find("five").unwrap() + 1;
        converted_string.insert(current_index, '5');

        if converted_string.contains("five") {
            current_index = converted_string.rfind("five").unwrap() + 1;
            converted_string.insert(current_index, '5');
        }
        word_found = true;
    }

    if converted_string.contains("six") {
        current_index = converted_string.find("six").unwrap() + 1;
        converted_string.insert(current_index, '6');

        if converted_string.contains("six") {
            current_index = converted_string.rfind("six").unwrap() + 1;
            converted_string.insert(current_index, '6');
        }
        word_found = true;
    }

    if converted_string.contains("seven") {
        current_index = converted_string.find("seven").unwrap() + 1;
        converted_string.insert(current_index, '7');

        if converted_string.contains("seven") {
            current_index = converted_string.rfind("seven").unwrap() + 1;
            converted_string.insert(current_index, '7');
        }
        word_found = true;
    }

    if converted_string.contains("eight") {
        current_index = converted_string.find("eight").unwrap() + 1;
        converted_string.insert(current_index, '8');

        if converted_string.contains("eight") {
            current_index = converted_string.rfind("eight").unwrap() + 1;
            converted_string.insert(current_index, '8');
        }
        word_found = true;
    }

    if converted_string.contains("nine") {
        current_index = converted_string.find("nine").unwrap() + 1;
        converted_string.insert(current_index, '9');

        if converted_string.contains("nine") {
            current_index = converted_string.rfind("nine").unwrap() + 1;
            converted_string.insert(current_index, '9');
        }
        word_found = true;
    }
    
    return converted_string;
} 