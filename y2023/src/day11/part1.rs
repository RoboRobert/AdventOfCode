use std::{fs, num};

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

    //Doubles their sizes
    let mut offset: usize = 0;
    for i in 0..empty_rows.len() {
        contents.insert(empty_rows[i] + offset, contents[empty_rows[i] + offset].to_string());
        offset += 1;
    }

    //Finds the empty columns
    let mut empty_cols: Vec<usize> = Vec::new();
    let mut current_string: String = String::from("");
    for i in 0..contents[0].len() {
        current_string = String::from("");
        for j in 0..contents.len() {
            current_string.push_str(contents[j].chars().nth(i).unwrap().to_string().as_str());
        }

        if !current_string.contains('#') {
            empty_cols.push(i);
        }
    }

    //Doubles their sizes
    offset = 0;
    for i in 0..empty_cols.len() {
        let current_col = empty_cols[i];
        for j in 0..contents.len() {
            contents[j].insert(current_col + offset, '.');
        }
        offset += 1;
    }

    for i in 0..contents.len() {
        for j in 0..contents[0].len() {
            if contents[i].chars().nth(j).unwrap() == '#' {
                let new_galaxy:galaxy = galaxy { row: i, col: j };
                galaxies.push(new_galaxy);
            }
        }
    }

    let mut shortest_paths: Vec<u64> = Vec::new();

    for i in 0..galaxies.len()-1 {
        for j in i+1..galaxies.len() {
            shortest_paths.push(shortest_path(&galaxies[i], &galaxies[j]));
        }
    }

    let mut total_paths: u64 = 0;
    for current in shortest_paths {
        total_paths += current;
    }

    println!("Total shortest paths: {}", total_paths);
}

fn shortest_path(gal1: &galaxy, gal2: &galaxy) -> u64 {
    let horizontal = gal1.col.abs_diff(gal2.col);
    let row = gal1.row.abs_diff(gal2.row);
    let total = horizontal + row;

    return total as u64;
}