use std::{fs, u64::MAX};

#[derive(Debug)]
struct map_seed {
    from_start: u64,
    to_start: u64,
    to_range: u64,
}

fn main() {
    
    //Read the file into a string
    let contents = fs::read_to_string("input.txt")
        .expect("File read properly!");

    //Stores the different seeds into a vector
    let seeds_str_vec: Vec<&str> = contents.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().collect();
    let mut seeds_vec: Vec<u64> = Vec::new();
    for current_seed in seeds_str_vec {
        seeds_vec.push(current_seed.parse::<u64>().unwrap());
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
            to_start: current_line.split_whitespace().nth(0).unwrap().parse::<u64>().unwrap(), 
            from_start: current_line.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap(), 
            to_range: current_line.split_whitespace().nth(2).unwrap().parse::<u64>().unwrap()
        };

        // dbg!(seed_mapper(&current_map, seeds_vec[0]));

        seed_maps[iterator-1].push(current_map);
    }

    let mut lowest_dest: u64 = MAX;
    let mut current_dest: u64;

    for seed in seeds_vec {
        current_dest = seed;

        for map_type in &seed_maps {

            for map in map_type {
                if !seed_mapper(map, current_dest).is_none() {
                    current_dest = seed_mapper(map, current_dest).unwrap();
                    break;
                }
            }
        }

        if current_dest < lowest_dest {
            lowest_dest = current_dest;
        }
    }

    println!("Lowest Destination: {}", lowest_dest);
}


fn seed_mapper(map:&map_seed, value: u64) -> Option<u64> {
    if value > map.from_start+map.to_range || value < map.from_start {
        return None;
    }

    return Some((map.to_start) + (value-map.from_start));
}