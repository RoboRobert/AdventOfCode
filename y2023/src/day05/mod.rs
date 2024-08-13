use std::{fs, u64::MAX};

#[derive(Debug)]
struct SeedMap {
    source_start: u64,
    destination_start: u64,
    range: u64,
}

// Tuple of starting seed value and number in range
#[derive(Debug)]
#[derive(Clone)]
struct SeedTuple {
    start: u64,
    range: u64,
}

#[derive(Debug)]
#[derive(Clone)]
struct MappedSeed {
    mapped: SeedTuple,
    unmapped: Vec<SeedTuple>,
}

pub fn puzzle1(input: &str) -> u64 {
    //Stores the different seeds into a vector
    let seeds_vec: Vec<SeedTuple> = input.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().map(|digit| digit.parse::<u64>().unwrap()).map(|integer| SeedTuple{start: integer, range: 1}).collect();

    let mut iterator: usize = 0;
    let mut seed_maps: Vec<Vec<SeedMap>> = Vec::new();
    for current_line in input.lines() {
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

        let current_map = SeedMap {
            destination_start: current_line.split_whitespace().nth(0).unwrap().parse::<u64>().unwrap(), 
            source_start: current_line.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap(), 
            range: current_line.split_whitespace().nth(2).unwrap().parse::<u64>().unwrap()
        };

        seed_maps[iterator-1].push(current_map);
    }

    let mut current_maps: Vec<SeedTuple> = seeds_vec;

    for map_vec in seed_maps {
        let mut next_maps: Vec<SeedTuple> = Vec::new();

        while current_maps.len() > 0 {
            let seed = current_maps.pop().unwrap();
            let mut mapped = false;
            for map in &map_vec {
                let mut value = seed_mapper(&seed, map);
                if value.mapped.range > 0 {
                    next_maps.push(value.mapped);
                    current_maps.append(&mut value.unmapped);
                    mapped = true;
                }
            }
            if mapped == false {
                next_maps.push(seed)
            }
        }
        
        current_maps = next_maps;
    }

    return current_maps.iter().map(|seed| seed.start).min().unwrap();
}

// Takes a seed and a seedmap and returns up to 3 new seed ranges, representing the mapped and unmapped areas
fn seed_mapper(seed: &SeedTuple, map: &SeedMap) -> MappedSeed {
    let mut unmapped: Vec<SeedTuple> = Vec::new();

    let mut overlap: SeedTuple = SeedTuple { start: 0, range: 0 };
    //If the ranges do not overlap, return the original seed range as overlap
    if seed.start > (map.source_start + map.range - 1) || map.source_start > (seed.start+seed.range - 1) {
        return MappedSeed {mapped: overlap, unmapped: vec![seed.clone()]};
    }

    //Otherwise, figure out where the overlap is
    if seed.start >= map.source_start {
        overlap.start = seed.start;
    }
    else if map.source_start > seed.start  {
        overlap.start = map.source_start;
    }

    if seed.start + seed.range <= map.source_start + map.range  {
        overlap.range = seed.start + seed.range - overlap.start;
    }
    else if map.source_start + map.range < seed.start + seed.range  {
        overlap.range = map.source_start + map.range - overlap.start;
    }
    
    // Push the overlapped range as mapped.
    let mapped: SeedTuple = SeedTuple{start: (map.destination_start) + (overlap.start-map.source_start), range: overlap.range};

    // Push the non-overlapping regions.
    if overlap.start > seed.start {
        unmapped.push(SeedTuple{start: seed.start, range: overlap.start - seed.start})
    }
    if (overlap.start + overlap.range) < (seed.start + seed.range) {
        let range: u64 = (seed.start + seed.range) - (overlap.start + overlap.range);
        unmapped.push(SeedTuple{start: overlap.start + overlap.range, range: range})
    }

    return MappedSeed {mapped: mapped, unmapped: unmapped};
}

pub fn puzzle2(input: &str) -> u64 {
    // let seed: SeedTuple = SeedTuple { start: 56, range: 1 };
    // let map: SeedMap = SeedMap { source_start: 20, destination_start: 0, range: 37 };
    
    // dbg!(seed_mapper(&seed, &map).mapped);
    // dbg!(seed_mapper(&seed, &map).unmapped);

    // return 0;

    //Stores the different seed ranges into a vector of pairs
    let seeds_str_vec: Vec<&str> = input.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().collect();
    let mut seeds_vec: Vec<SeedTuple> = Vec::new();

    let mut iterator = 0;
    while iterator < seeds_str_vec.len()-1 {
        let current_seed = SeedTuple {start: seeds_str_vec[iterator].parse::<u64>().unwrap(), range: seeds_str_vec[iterator+1].parse::<u64>().unwrap()};
        seeds_vec.push(current_seed);

        iterator+= 2;
    }

    let mut iterator: usize = 0;
    let mut seed_maps: Vec<Vec<SeedMap>> = Vec::new();
    for current_line in input.lines() {
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

        let current_map = SeedMap {
            destination_start: current_line.split_whitespace().nth(0).unwrap().parse::<u64>().unwrap(), 
            source_start: current_line.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap(), 
            range: current_line.split_whitespace().nth(2).unwrap().parse::<u64>().unwrap()
        };

        seed_maps[iterator-1].push(current_map);
    }

    let mut current_maps: Vec<SeedTuple> = seeds_vec;

    for map_vec in seed_maps {
        let mut next_maps: Vec<SeedTuple> = Vec::new();

        // dbg!(&current_maps.len());
        while current_maps.len() > 0 {
            let seed = current_maps.pop().unwrap();
            // dbg!(&seed);
            let mut mapped = false;
            for map in &map_vec {
                let mut value = seed_mapper(&seed, map);
                if value.mapped.range > 0 {
                    next_maps.push(value.mapped);
                    current_maps.append(&mut value.unmapped);
                    mapped = true;
                }
            }
            if mapped == false {
                next_maps.push(seed)
            }
        }
        
        current_maps = next_maps;
    }

    return current_maps.iter().map(|seed| seed.start).min().unwrap();
}