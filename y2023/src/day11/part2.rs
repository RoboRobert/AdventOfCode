use std::{fs, num, io::empty};

#[derive(Debug)]
struct galaxy {
    row: usize,
    col: usize,
}

fn main() {
    let binding = fs::read_to_string("input.txt").expect("File not opened properly!");
    let mut contents: Vec<String> = Vec::new();
    for line in binding.lines() {
        contents.push(line.to_string());
    }

    let mut galaxies: Vec<galaxy> = Vec::new();

    //Finds the empty rows
    let mut empty_rows: Vec<usize> = Vec::new();
    for i in 0..contents.len() {
        if !contents[i].contains('#') {
            empty_rows.push(i);
        }
    }

    //Finds the empty columns
    let mut empty_cols: Vec<usize> = Vec::new();
    let mut current_string: String = String::from("");
    for i in 0..contents[0].len() {
        current_string = String::from("");
        for j in 0..contents.len() {
            //Finds the locations of the galaxies
            if contents[j].chars().nth(i).unwrap() == '#' {
                let new_galaxy:galaxy = galaxy { row: j, col: i };
                galaxies.push(new_galaxy);
            }
            current_string.push_str(contents[j].chars().nth(i).unwrap().to_string().as_str());
        }

        if !current_string.contains('#') {
            empty_cols.push(i);
        }
    }

    //Vector to store shortest paths
    let mut shortest_paths: Vec<u128> = Vec::new();

    //Goes through every pair of galaxies and finds the shortest path between them
    for i in 0..galaxies.len()-1 {
        for j in i+1..galaxies.len() {
            shortest_paths.push(shortest_path(&galaxies[i], &galaxies[j], &empty_rows,& empty_cols));
        }
    }

    //Then reads the shortest paths back out and sums them
    let mut total_paths: u128 = 0;
    for current in shortest_paths {
        total_paths += current;
    }

    println!("Total shortest paths: {}", total_paths);
}

fn shortest_path(gal1: &galaxy, gal2: &galaxy, empty_columns: &Vec<usize>, empty_rows: &Vec<usize>) -> u128 {
    let multiplier: usize = 999999;

    let horizontal = gal1.col.abs_diff(gal2.col);
    let row = gal1.row.abs_diff(gal2.row);
    let mut total = horizontal + row;

    //Checks if the path goes over any empty columns
    for position in empty_columns {
        if contained_in(&gal1.row, &gal2.row, position) {
            total+=multiplier;
        }
    }

    //Checks if the path goes over any empty rows
    for position in empty_rows {
        if contained_in(&gal1.col, &gal2.col, position) {
            total+=multiplier;
        }
    }

    return total as u128;
}

fn contained_in(start: &usize, end: &usize, value: &usize) -> bool {
    let mut least_start: usize = *start;
    let mut greatest_end: usize = *end;
    if start >= end {
        least_start = *end;
        greatest_end = *start;
    }

    if *value > greatest_end || *value < least_start {
        return false;
    }

    return true;
}