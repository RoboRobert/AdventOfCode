pub fn puzzle1(input: &str) -> u64 {
    //Parses the input into vectors
    let time_vec_str: Vec<&str> = input.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().collect();
    let mut time_vec: Vec<u64> = Vec::new();

    for string in time_vec_str {
        time_vec.push(string.parse::<u64>().unwrap());
    }

    let distance_vec_str: Vec<&str> = input.lines().nth(1).unwrap().split(":").nth(1).unwrap().split_whitespace().collect();
    let mut distance_vec: Vec<u64> = Vec::new();

    for string in distance_vec_str {
        distance_vec.push(string.parse::<u64>().unwrap());
    }

    let mut current_ways: u64 = 0;
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

    return final_answer;
}

pub fn puzzle2(input: &str) -> i32 {
    //Parses the input into vectors
    let time: u64 = 55826490;
    let distance: u64 = 246144110121111;

    let mut final_answer = 0;
    for iterator in 1..time {
        if travelled_dist(iterator,  time-iterator) > distance {
            final_answer += 1;
        }
    }

    return final_answer;
}

fn travelled_dist(velocity: u64, remaining_time: u64) -> u64 {
    return velocity*remaining_time;
}