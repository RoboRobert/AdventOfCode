use std::fs;

fn main() {
    //Read the file into a string
    let contents = fs::read_to_string("input.txt")
        .expect("File read properly!");

    //Parses the input into vectors
    let time_vec_str: Vec<&str> = contents.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().collect();
    let mut time_vec: Vec<u32> = Vec::new();

    for string in time_vec_str {
        time_vec.push(string.parse::<u32>().unwrap());
    }

    let distance_vec_str: Vec<&str> = contents.lines().nth(1).unwrap().split(":").nth(1).unwrap().split_whitespace().collect();
    let mut distance_vec: Vec<u32> = Vec::new();

    for string in distance_vec_str {
        distance_vec.push(string.parse::<u32>().unwrap());
    }

    let mut current_ways: u32 = 0;
    let mut final_answer = 1;
    for index in 0..time_vec.len() {
        current_ways = 0;
        for iterator in 1..time_vec[index] {
            if travelled_dist(iterator, time_vec[index]-iterator) > distance_vec[index] {
                current_ways += 1;
            }
        }

        final_answer*=current_ways;
    }

    println!("Final answer: {}", final_answer);
}

fn travelled_dist(velocity: u32, remaining_time: u32) -> u32 {
    return velocity*remaining_time;
}