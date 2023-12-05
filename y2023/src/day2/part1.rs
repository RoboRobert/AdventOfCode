use std::{fs, thread::current};

fn main()  {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let mut index_sum = 0;

    let file_path = String::from("input.txt");

    //Read the file into a string
    let contents = fs::read_to_string(file_path)
        .expect("File read properly!");
    
    let mut possible: bool = true;
    for word in contents.lines() {
        possible = true;
        let line_vec: Vec<&str> = word.split([',', ':', ';']).collect();

        let index = line_vec[0].split(' ').last().unwrap().parse::<i32>().unwrap();

        for line_index in 1..line_vec.len() {
            let color = line_vec[line_index].split(' ').nth(2).unwrap();
            let current_cubes = line_vec[line_index].split(' ').nth(1).unwrap().parse::<i32>().unwrap();
            if color == "blue" && current_cubes > blue_cubes || color == "green" && current_cubes > green_cubes || color == "red" && current_cubes > red_cubes {
                possible = false;
            }
        }

        if possible == true {
            index_sum += index;
        }
    }

    println!("Sum of possible indices: {}", index_sum);
}