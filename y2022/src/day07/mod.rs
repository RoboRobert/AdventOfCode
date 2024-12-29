use std::{collections::HashMap, thread::current};

fn add_size(size: i128, path: &str, map: &mut HashMap<String, i128>) {
    let mut current = path;
    while current != "" {
        *map.entry(current.to_owned()).or_insert(0) += size;
        current = &current[0..current.rfind('/').unwrap_or(0)];
    }
}

fn get_directories(input:&str) -> HashMap<String, i128> {
    let mut dir_map: HashMap<String, i128> = HashMap::new();
    
    let mut current_path = String::from("/");
    for ele in input.lines().enumerate() {
        if(ele.1 == "$ cd /") {
            continue;
        }
        println!("{current_path}");
        let mut split = ele.1.split(" ");
        let first = split.nth(0).unwrap();
        let second = split.nth(0).unwrap();
        let third = split.nth(0);
        // Handle adding file sizes
        if (first.parse::<i128>().is_ok()) {
            add_size(first.parse::<i128>().unwrap(), &current_path, &mut dir_map);
        }
        // Handle directory change
        else if(second == "cd") {
            let dir = third.unwrap();
            // dbg!(current_parent);
            match dir {
                ".." => {
                    current_path = current_path[0..current_path.rfind('/').unwrap()].to_string();
                },
                _ => {
                    let mut add = String::from("/");
                    add += dir;
                    current_path += add.as_str();
                },
            }
        }
        
    }

    dir_map
}

pub fn puzzle1(input: &str) -> i128 {
    let dir_map = get_directories(input);

    dir_map.values().filter(|&val| val <= &100000).sum()
}

pub fn puzzle2(input: &str) -> i128 {
    let dir_map = get_directories(input);

    let total_size: i128 = 70000000;
    let space_needed: i128 = 30000000;
    let current_space: i128 = total_size - *dir_map.get("/").unwrap();

    dbg!(current_space);

    let min_diff = space_needed - current_space;

    *dir_map.values().filter(|&val| *val > min_diff).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_07_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 95437);
    }

    #[test]
    fn test_day_07_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1391690);
    }

    #[test]
    fn test_day_07_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 24933642);
    }

    #[test]
    fn test_day_07_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 5469168);
    }
}