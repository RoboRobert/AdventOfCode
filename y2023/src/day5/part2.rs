use std::{fs, u64::MAX};

#[derive(Debug)]
struct map_seed {
    destination_start: u64,
    source_start: u64,
    range: u64,
}

#[derive(Debug)]
struct source_range(u64, u64);

#[derive(Debug)]
struct seed_tuple(u64, u64);

fn main() {
    
    //Read the file into a string
    let contents = fs::read_to_string("example.txt")
        .expect("File read properly!");

    //Stores the different seed ranges into a vector of pairs
    let seeds_str_vec: Vec<&str> = contents.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().collect();
    let mut seeds_vec: Vec<seed_tuple> = Vec::new();

    let mut iterator = 0;
    while iterator < seeds_str_vec.len()-1 {
        let current_seed = seed_tuple(seeds_str_vec[iterator].parse::<u64>().unwrap(), seeds_str_vec[iterator+1].parse::<u64>().unwrap());
        seeds_vec.push(current_seed);

        iterator+= 2;
    }

    let mut iterator: usize = 0;
    let mut seed_maps: Vec<Vec<map_seed>> = Vec::new();
    for current_line in contents.lines() {
        //If the current line is empty or useless, keep going.
        if current_line.is_empty() || current_line.contains("seeds:") {
            continue;
        }

        //Each time I find a new map, push
        if current_line.contains("map") {
            seed_maps.push(Vec::new());
            iterator += 1;
            continue;
        }

        let current_map = map_seed { 
            destination_start: current_line.split_whitespace().nth(0).unwrap().parse::<u64>().unwrap(), 
            source_start: current_line.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap(), 
            range: current_line.split_whitespace().nth(2).unwrap().parse::<u64>().unwrap()
        };

        // dbg!(seed_mapper(&current_map, seeds_vec[0]));

        seed_maps[iterator-1].push(current_map);
    }

    let mut lowest_dest: u64 = MAX;
    let mut current_range: source_range;

    // for seed in seeds_vec {
        current_range = source_range(seeds_vec[0].0, seeds_vec[0].1);
        dbg!(&current_range);

        for map_type in &seed_maps {
            dbg!(&current_range);
            for map in map_type {
                let current_map = source_range(map.source_start, map.range);
                if !contained_in(&current_range, &current_map).is_none() {
                    dbg!(&current_map);
                    let mut temp_range = contained_in(&current_range, &current_map).unwrap();
                    dbg!(&temp_range);
                    temp_range.0 = seed_mapper(&map, temp_range.0).unwrap();
                    // dbg!(&temp_range);
                    current_range = temp_range;
                    
                    break;
                }
            }
        }

        if current_range.0 < lowest_dest {
            lowest_dest = current_range.0;
        }
    // }

    println!("Lowest Destination: {}", lowest_dest);
}

fn seed_mapper(map:&map_seed, value: u64) -> Option<u64> {
    if value > map.source_start+map.range || value < map.source_start {
        return None;
    }

    return Some((map.destination_start) + (value-map.source_start));
}

//Takes in two maps and returns the intersection slice as another map
fn contained_in(current: &source_range, checked: &source_range) -> Option<source_range> {
    let mut returned_range: source_range = source_range(0,0);
    //If the ranges do not overlap, return None
    if current.0 > (checked.0+checked.1) || checked.0 > (current.0+current.1) {
        return None;
    }

    //Otherwise, figure out where the overlap is
    if current.0 >= checked.0 {
        returned_range.0 = current.0;
    }
    else if checked.0 > current.0  {
        returned_range.0 = checked.0;
    }

    if current.0 + current.1 <= checked.0 + checked.1  {
        returned_range.1 = current.0 + current.1 - returned_range.0;
    }
    else if checked.0 + checked.1 < current.0 + current.1  {
        returned_range.1 = checked.0 + checked.1 - returned_range.0;
    }

    return Some(returned_range);
}