use std::{fs, thread::current};

fn main()  {
    let mut power_sum = 0;

    let file_path = String::from("input.txt");

    //Read the file into a string
    let contents = fs::read_to_string(file_path)
        .expect("File read properly!");
    
    for word in contents.lines() {
        let line_vec: Vec<&str> = word.split([',', ':', ';']).collect();

        let mut greatest_blue = 0;
        let mut greatest_green = 0;
        let mut greatest_red = 0;
        for line_index in 1..line_vec.len() {
            let color = line_vec[line_index].split(' ').nth(2).unwrap();
            let current_cubes = line_vec[line_index].split(' ').nth(1).unwrap().parse::<i32>().unwrap();
            match color {
                "blue" => {
                    if current_cubes > greatest_blue {
                        greatest_blue = current_cubes;
                    }
                }
                "red" => {
                    if current_cubes > greatest_red {
                        greatest_red = current_cubes;
                    }
                }
                "green" => {
                    if current_cubes > greatest_green {
                        greatest_green = current_cubes;
                    }
                }
                &_ => {println!("Weird!");}
            }
        }

        let current_power = greatest_blue*greatest_green*greatest_red;

        power_sum += current_power;
    }

    println!("Sum of cube powers: {}", power_sum);
}