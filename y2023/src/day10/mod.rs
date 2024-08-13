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

struct Coord {
    x: i128,
    y: i128,
}

fn traverse_loop(input: &str) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();

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
    let mut looped: bool = false;

    let mut dir = direction::right;
    let mut entrance_dir = entrance::vertical;
    while looped == false {
        let mut current_char = char_vec[current_row][current_column];
        //In my input, the S is a horizontal pipe
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

        coords.push(Coord { x: current_row as i128, y: current_column as i128 });

        if char_vec[current_row][current_column] == 'S' {
            looped = true;
        }
    }

    return coords;
}

pub fn puzzle1(input: &str) -> i128 {
    return (traverse_loop(input).len() as i128)/2;
}

pub fn puzzle2(input: &str) -> i128 {
    let coords: Vec<Coord> = traverse_loop(input);
    let b: i128 = coords.len() as i128;
    let mut area: i128 = 0;

    for current in coords.iter().enumerate() {
        let mut x1: i128 = 0;
        let mut x2: i128 = 0;
        if current.0 == 0 {x1 = coords.last().unwrap().x}
        else {x1 = coords[current.0 - 1].x}
        if current.0 == coords.len()-1 {x2 = coords.first().unwrap().x}
        else {x2 = coords[current.0 + 1].x}

        area += current.1.y * (x1 - x2);
    }
    area = area/2;

    let i = area - (b/2) + 1;

    return i;
}