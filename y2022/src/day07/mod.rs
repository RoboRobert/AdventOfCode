use std::collections::HashMap;

fn add_size(size: i128, dir: &str, map: &mut HashMap<&str, Dir>) {
    let mut current = dir;
    while current != "" {
        let current_dir = map.get_mut(current).unwrap();
        current_dir.size += size;
        current = current_dir.parent;
    }
}

fn get_directories(input:&str) -> HashMap<&str, i128> {
    let mut dir_map: HashMap<&str, i128> = HashMap::new();

    dir_map.insert("/", Dir {name: "/", parent: "", size: 0 });
    
    let mut current = Dir {name: "", parent: "", size: 0 };
    for ele in input.lines().enumerate() {
        let name = current.name;
        println!("{name}");
        let mut split = ele.1.split(" ");
        let first = split.nth(0).unwrap();
        let second = split.nth(0).unwrap();
        let third = split.nth(0);
        // Handle adding file sizes
        if (first.parse::<i128>().is_ok()) {
            add_size(first.parse::<i128>().unwrap(), current.name, &mut dir_map);
        }
        // Handle directory change
        else if(second == "cd") {
            let dir = third.unwrap();
            let current_parent = current.parent;
            // dbg!(current_parent);
            match dir {
                ".." => {
                    if(dir_map.contains_key(current_parent)) {
                        current = dir_map.get(&current_parent).unwrap().clone();
                    }
                },
                _ => {
                    current = *dir_map.entry(dir).or_insert(Dir { parent: current.name, name:dir, size: 0 });
                },
            }
        }
        
    }

    dir_map
}

pub fn puzzle1(input: &str) -> i128 {
    let dir_map = get_directories(input);

    // for ele in &dir_map {
    //     dbg!(ele.1);
    // }

    dir_map.values().filter(|val| val.size <= 100000).map(|val| val.size).sum()
}

pub fn puzzle2(input: &str) -> i128 {
    let dir_map = get_directories(input);

    let size_needed: i128 = 30000000;
    let current_size: i128 = dir_map.get("/").unwrap().size;

    let min_diff = current_size - size_needed;

    dir_map.values().filter(|val| val.size >= min_diff).map(|val| val.size).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_07_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 11);
    }

    #[test]
    fn test_day_07_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1034);
    }

    #[test]
    fn test_day_07_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 26);
    }

    #[test]
    fn test_day_07_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 2472);
    }
}