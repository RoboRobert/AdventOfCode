use std::{fs, thread::current};

#[derive(PartialEq)]
enum entrance {
    vertical,
    side,
}

#[derive(PartialEq)]
enum direction {
    right,
    left
}

pub fn puzzle1(input: &str) -> u128 {
    let contents = fs::read_to_string("input.txt").expect("File not opened properly!");

    let mut char_vec: Vec<Vec<char>> = Vec::new();

    let mut temp: usize = 0;

    let mut current_row: usize = 0;
    let mut current_column: usize = 0;
    //Stores the input into a 2D character array
    for line in contents.lines() {
        if line.find('S') != None {
            current_column = line.find('S').unwrap();
            current_row = temp;
        }

        let vec: Vec<char> = line.chars().collect();
        char_vec.push(vec);
        temp += 1;
    }

    let mut steps: u128 = 0;

    //Direction around the loop. 1 is right, 0 is left

    let mut looped: bool = false;

    let mut dir = direction::right;
    let mut entrance_dir = entrance::vertical;
    while looped == false {
        let mut current_char = char_vec[current_row][current_column];
        //In my input, the S is a straight pipe
        if current_char == 'S' {
            current_char = '-';
        }

        match current_char {
            '|' => {
                if dir == direction::right {
                    current_row += 1;
                }
                else if dir == direction::left {
                    current_row -= 1;
                }
                
                entrance_dir = entrance::vertical;
            } 

            '-' => {
                if dir == direction::right {
                    current_column += 1;
                }
                else if dir == direction::left {
                    current_column -= 1;
                }

                entrance_dir = entrance::side;
            }

            'L' => {
                if entrance_dir == entrance::side {
                    current_row -= 1;
                    entrance_dir = entrance::vertical;
                    dir = direction::left;
                }
                else if entrance_dir == entrance::vertical {
                    current_column += 1;
                    entrance_dir = entrance::side;
                    dir = direction::right;
                }
            } 

            'F' => {
                if entrance_dir == entrance::side {
                    current_row += 1;
                    entrance_dir = entrance::vertical;
                    dir = direction::right;
                }
                else if entrance_dir == entrance::vertical {
                    current_column += 1;
                    entrance_dir = entrance::side;
                    dir = direction::right;
                }
            }

            'J' => {
                if entrance_dir == entrance::side {
                    current_row -= 1;
                    entrance_dir = entrance::vertical;
                    dir = direction::left;
                }
                else if entrance_dir == entrance::vertical {
                    current_column -= 1;
                    entrance_dir = entrance::side;
                    dir = direction::left;
                }
            } 

            '7' => {
                if entrance_dir == entrance::side {
                    current_row += 1;
                    entrance_dir = entrance::vertical;
                    dir = direction::right;
                }
                else if entrance_dir == entrance::vertical {
                    current_column -= 1;
                    entrance_dir = entrance::side;
                    dir = direction::left;
                }
            }
            
            _ => {println!("Weird!");}
        }

        steps += 1;

        if char_vec[current_row][current_column] == 'S' {
            looped = true;
        }
    }

    let furthest = steps/2;

    return furthest;
}

pub fn puzzle2(input: &str) -> u128 {
    let mut char_vec: Vec<Vec<char>> = Vec::new();

    let mut temp: usize = 0;

    let mut current_row: usize = 0;
    let mut current_column: usize = 0;
    //Stores the input into a 2D character array
    for line in input.lines() {
        if line.find('S') != None {
            current_column = line.find('S').unwrap();
            current_row = temp;
        }

        let vec: Vec<char> = line.chars().collect();
        char_vec.push(vec);
        temp += 1;
    }

    //Direction around the loop. 1 is right, 0 is left

    let mut looped: u32 = 0;

    let mut dir = direction::right;
    let mut entrance_dir = entrance::vertical;

    let mut char_vec_marked = char_vec.clone();
    //This goes around the loop twice. Once to mark the characters, and another time
    //to mark the inside characters.
    while looped < 2 {
        let mut current_char = char_vec[current_row][current_column];
        //In my input, the S is a straight pipe
        if current_char == 'S' {
            current_char = 'F';
        }

        match current_char {
            '|' => {
                char_vec_marked[current_row][current_column] = 'P';
                if dir == direction::right {
                    current_row += 1;
                }
                else if dir == direction::left {
                    current_row -= 1;
                }
                
                entrance_dir = entrance::vertical;
            } 

            '-' => {
                char_vec_marked[current_row][current_column] = 'P';
                if dir == direction::right {
                    if looped == 1 {
                        search_dir(&mut char_vec_marked, current_row, current_column, 1);
                    }
                    // char_vec_marked[current_row][current_column] = 'R';
                    current_column += 1;
                }
                else if dir == direction::left {
                    if looped == 1 {
                        search_dir(&mut char_vec_marked, current_row, current_column, 0);
                    }
                    // char_vec_marked[current_row][current_column] = 'L';
                    current_column -= 1;
                }

                entrance_dir = entrance::side;
            }

            'L' => {
                char_vec_marked[current_row][current_column] = 'P';
                if entrance_dir == entrance::side {
                    current_row -= 1;
                    entrance_dir = entrance::vertical;
                    dir = direction::left;
                }
                else if entrance_dir == entrance::vertical {
                    current_column += 1;
                    entrance_dir = entrance::side;
                    dir = direction::right;
                }
            } 

            'F' => {
                char_vec_marked[current_row][current_column] = 'P';
                if entrance_dir == entrance::side {
                    current_row += 1;
                    entrance_dir = entrance::vertical;
                    dir = direction::right;
                }
                else if entrance_dir == entrance::vertical {
                    current_column += 1;
                    entrance_dir = entrance::side;
                    dir = direction::right;
                }
            }

            'J' => {
                char_vec_marked[current_row][current_column] = 'P';
                if entrance_dir == entrance::side {
                    current_row -= 1;
                    entrance_dir = entrance::vertical;
                    dir = direction::left;
                }
                else if entrance_dir == entrance::vertical {
                    current_column -= 1;
                    entrance_dir = entrance::side;
                    dir = direction::left;
                }
            } 

            '7' => {
                char_vec_marked[current_row][current_column] = 'P';
                if entrance_dir == entrance::side {
                    current_row += 1;
                    entrance_dir = entrance::vertical;
                    dir = direction::right;
                }
                else if entrance_dir == entrance::vertical {
                    current_column -= 1;
                    entrance_dir = entrance::side;
                    dir = direction::left;
                }
            }
            
            _ => {println!("Weird!");}
        }

        if char_vec[current_row][current_column] == 'S' {
            looped +=1;
        }
    }

    for vector in &char_vec_marked {
        for character in vector {
            print!("{character}");
        }
        println!();
    }
    
    let mut total_inside: u128 = 0;
    for vector in &char_vec_marked {
        for character in vector {
            if *character == 'I' {
                total_inside +=1;
            }
        }
    }

    return total_inside;
}

fn search_dir(input_vec: &mut Vec<Vec<char>>, row: usize, col: usize, direction: u32) {
    match direction {
        //Up
        0 => {
            if row == 0 { return; }
            for current_row in (0..(row)).rev() {
                if input_vec[current_row][col] == 'P' {
                    break;
                }
                input_vec[current_row][col] = 'I';
            }
        }

        //Down
        1 => {
            if row == input_vec.len() { return; }
            for current_row in (row+1)..input_vec.len() {
                if input_vec[current_row][col] == 'P' {
                    break;
                }
                input_vec[current_row][col] = 'I';
            }
        }

        //Default case
        _ => {println!("Weird!");}
    }
}