use std::fs;

fn main() {
    //Read the file into a string
    let contents = fs::read_to_string("input.txt")
        .expect("File read properly!");

    //Parses the input into vectors
    let time: u64 = 55826490;
    let distance: u64 = 246144110121111;

    let mut final_answer = 0;
    for iterator in 1..time {
        if travelled_dist(iterator,  time-iterator) > distance {
            final_answer += 1;
        }
    }

    println!("Final answer: {}", final_answer);
}

fn travelled_dist(velocity: u64, remaining_time: u64) -> u64 {
    return velocity*remaining_time;
}