use std::{fs, thread::current};

fn main()  {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let mut possible: bool = true;

    let mut index = 0;
    let mut index_sum = 0;
    let mut current_cubes = 0;

    let file_path = String::from("input.txt");

    //Read the file into a string
    let contents = fs::read_to_string(file_path)
        .expect("File read properly!");
    
    for word in contents.lines() {
        possible = true;
        let line_vec: Vec<&str> = word.split([',', ':', ';']).collect();

        index = line_vec[0].split(' ').last().unwrap().parse::<i32>().unwrap();

        for piece in line_vec {
            if piece.contains("blue") {
                current_cubes = piece.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
                print!("Blue: {} ", current_cubes);

                if current_cubes > blue_cubes {
                    possible = false;
                }
            }

            else if piece.contains("green") {
                current_cubes = piece.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
                print!("Green: {} ", current_cubes);

                if current_cubes > green_cubes {
                    possible = false;
                }
            }

            else if piece.contains("red") {
                current_cubes = piece.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
                print!("Red: {} ", current_cubes);

                if current_cubes > red_cubes {
                    possible = false;
                }
            }
        }

        if possible == true {
            index_sum += index;
        }

        println!{""};
    }

    println!("Sum of possible indices: {}", index_sum);
}