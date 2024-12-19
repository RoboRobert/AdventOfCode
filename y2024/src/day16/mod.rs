use std::{collections::HashMap, hash::Hash};

fn dijkstra(weight_map: &mut HashMap<(isize, isize), i128>, start: (isize, isize), end: (isize, isize)) -> Vec<(isize,isize)> {
    let mut seen_vec: Vec<(isize,isize)> = Vec::new();
    let mut node_vec: Vec<(isize,isize)> = Vec::new();

    let mut pos = start;

    let mut neighbor_vec: Vec<(isize,isize)> = Vec::new();
    // Push up, down, left, right
    neighbor_vec.push((pos.0-1, pos.1));
    neighbor_vec.push((pos.0+1, pos.1));
    neighbor_vec.push((pos.0, pos.1-1));
    neighbor_vec.push((pos.0, pos.1+1));

    

    return node_vec;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let char_vec: Vec<Vec<char>> = input.split("\n\n").nth(0).unwrap().lines().map(|line| line.chars().collect()).collect();

    let mut weight_map: HashMap<(isize, isize), i128> = HashMap::new();
    let mut start: (isize, isize) = (0,0);
    let mut end: (isize, isize) = (0,0);
    for row in char_vec.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            let pos = (row.0 as isize, col.0 as isize);
            let mut weight: i128 = 1;
            match *col.1 {
                'S' => {weight = 0; start = pos;}
                'E' => {weight = 1; end = pos;}
                '.' => weight = 1,
                '#' => weight = 1000000,
                _ => {dbg!("WEIRD");}
            }
            weight_map.insert(pos, weight);
        }
    }

    dbg!(dijkstra(&mut weight_map, start, end));

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum: i128 = 0;

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_16_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 1930);
    }

    #[test]
    fn test_day_16_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1352976);
    }

    #[test]
    fn test_day_16_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 81);
    }

    #[test]
    fn test_day_16_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1366);
    }
}
