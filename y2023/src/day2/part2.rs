use std::{fs, thread::current};

fn main()  {
    let mut greatest_blue = 0;
    let mut greatest_green = 0;
    let mut greatest_red = 0;

    let mut current_cubes = 0;
    let mut current_power = 1;
    let mut power_sum = 0;

    let file_path = String::from("input.txt");

    //Read the file into a string
    let contents = fs::read_to_string(file_path)
        .expect("File read properly!");
    
    for word in contents.lines() {
        let line_vec: Vec<&str> = word.split([',', ':', ';']).collect();

        greatest_blue = 0;
        greatest_green = 0;
        greatest_red = 0;

        for piece in line_vec {
            if piece.contains("blue") {
                current_cubes = piece.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
                print!("Blue: {} ", current_cubes);

                if(current_cubes > greatest_blue) {
                    greatest_blue = current_cubes;
                }
            }

            else if piece.contains("green") {
                current_cubes = piece.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
                print!("Green: {} ", current_cubes);

                if(current_cubes > greatest_green) {
                    greatest_green = current_cubes;
                }
            }

            else if piece.contains("red") {
                current_cubes = piece.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
                print!("Red: {} ", current_cubes);

                if(current_cubes > greatest_red) {
                    greatest_red = current_cubes;
                }
            }
        }

        current_power = greatest_blue*greatest_green*greatest_red;

        power_sum += current_power;

        println!{""};
    }

    println!("Sum of cube powers: {}", power_sum);
}