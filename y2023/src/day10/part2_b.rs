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

fn main() {
    //567 too high
    let contents = fs::read_to_string("example.txt").expect("File not opened properly!");

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

    // let mut char_vec_marked = char_vec.clone();
    while looped == false {
        let mut current_char = char_vec[current_row][current_column];
        //In my input, the S is a straight pipe
        if current_char == 'S' {
            current_char = 'F';
        }

        match current_char {
            '|' => {
                char_vec[current_row][current_column] = 'P';
                if dir == direction::right {
                    current_row += 1;
                }
                else if dir == direction::left {
                    current_row -= 1;
                }
                
                entrance_dir = entrance::vertical;
            } 

            '-' => {
                char_vec[current_row][current_column] = 'P';
                if dir == direction::right {
                    current_column += 1;
                }
                else if dir == direction::left {
                    current_column -= 1;
                }

                entrance_dir = entrance::side;
            }

            'L' => {
                char_vec[current_row][current_column] = 'P';
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
                char_vec[current_row][current_column] = 'P';
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
                char_vec[current_row][current_column] = 'P';
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
                char_vec[current_row][current_column] = 'P';
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

        if char_vec[current_row][current_column] == 'P' {
            looped = true;
        }
    }

    for vector in &char_vec {
        for character in vector {
            print!("{character}");
        }
        println!();
    }

    let mut total_inside: u128 = 0;
    println!("Total Inside: {}", total_inside);
}

