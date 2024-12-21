use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    parent: String,
    size: i128,
}

pub fn puzzle1(input: &str) -> i128 {
    let mut dir_map: HashMap<&str, Dir> = HashMap::new();

    dir_map.insert("/", Dir {name: String::from("/"), parent: String::from(""), size: 0 });
    
    let mut current = *dir_map.get("/").unwrap();
    for ele in input.lines() {
        // when changing directory, set the current
        if(ele.split(" ").nth(0).unwrap() == "cd") {
            let dir = ele.split(" ").nth(1).unwrap();
            let current_parent = current.parent.clone();
            match dir {
                ".." => current = dir_map.get(&current_parent.as_str()).unwrap().clone(),
                _ => {current = Dir { parent: current.parent, name:dir.to_string(), size: 0 }}
            }
        }
    }

    0
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
