use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Dir<'a> {
    name: &'a str,
    parent: &'a str,
    size: i128,
}

fn add_size(size: i128, dir: &str, map: &mut HashMap<&str, Dir>) {
    let mut current = dir;
    while current != "" {
        let current_dir = map.get_mut(current).unwrap();
        current_dir.size += size;
        current = current_dir.parent;
    }
}

pub fn puzzle1(input: &str) -> i128 {
    let mut dir_map: HashMap<&str, Dir> = HashMap::new();

    dir_map.insert("", Dir {name: "", parent: "/", size: 0 });
    
    let mut current = Dir {name: "", parent: "/", size: 0 };
    for ele in input.lines() {
        // Handle directory change
        if(ele.split(" ").nth(1).unwrap() == "cd") {
            let dir = ele.split(" ").nth(2).unwrap();
            let current_parent = current.parent;
            dbg!(current);
            dbg!(dir);
            match dir {
                ".." => {
                    current = dir_map.get(&current_parent).unwrap().clone()
                },
                _ => {
                    dir_map.entry(dir).or_insert(Dir { parent: current.name, name:dir, size: 0 });
                    current = dir_map.get(&dir).unwrap().clone();
                },
            }
        }

        // Handle adding file sizes
        else if (ele.split(" ").nth(0).unwrap().parse::<i128>().is_ok()) {
            add_size(ele.split(" ").nth(0).unwrap().parse::<i128>().unwrap(), current.name, &mut dir_map);
        }
    }

    for ele in &dir_map {
        dbg!(ele.1);
    }

    dir_map.values().filter(|val| val.size < 100000).map(|val| val.size).sum()
}

pub fn puzzle2(input: &str) -> i128 {
    0
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
