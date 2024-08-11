use std::fs;

pub fn puzzle1(input: &str) -> i128 {
    let contents = fs::read_to_string("example.txt").expect("File not opened properly!\n");

    //Gathers the different patterns into a vector
    let pattern_vec: Vec<&str> = contents.split("\n\n").collect();
    let mut pattern_vec_transpose: Vec<String> = Vec::new();

    return 0;
}

pub fn puzzle2(input: &str) -> i128 {
    return 0;
}