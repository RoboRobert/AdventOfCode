use std::{fs, thread::current};

#[derive(Debug)]
struct spring_record {
    record: String,
    spring_order: Vec<u128>,
}

pub fn puzzle1(input: &str) -> u128 {
    //Parses each line into a more useful spring_record data format
    let mut record_vec: Vec<spring_record> = Vec::new();
    for line in input.lines() {
        let order_vec_binding: Vec<&str> = line.split_whitespace().nth(1).unwrap().split(',').collect();
        let mut order_vec: Vec<u128> = Vec::new();
        for current in order_vec_binding {
            order_vec.push(current.parse::<u128>().unwrap());
        }

        let current_record = line.split_whitespace().nth(0).unwrap();

        let new_record: spring_record = spring_record { record: current_record.to_string(), spring_order: order_vec };
        record_vec.push(new_record);
    }

    let mut sum: u128 = 0;

    return sum;
}

fn generate_hash(length: u128) -> String {
    let mut ret_string: String = String::from("");
    for current in 0..length {
        ret_string.push('#');
    }
    return ret_string;
}

fn generate_question(length: u128) -> String {
    let mut ret_string: String = String::from("");
    for current in 0..length {
        ret_string.push('?');
    }
    return ret_string;
}

// fn find_combinations(line: String, order_vec: Vec<u128>, index: usize) -> u128 {
//     let current_hash = generate_hash(order_vec[index]);
//     if line.contains(current_hash.as_str()) {

//     }

//     let current_pattern: String = generate_question(order_vec[index]);
// }

pub fn puzzle2(input: &str) -> i128 {
    return 0;
}