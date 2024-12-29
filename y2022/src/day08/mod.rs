use std::collections::HashMap;

fn scenic_score(start: (isize, isize), map: &HashMap<(isize, isize), usize>) -> i128 {
    let mut score: i128 = 1;
    let tree_val = map.get(&start).unwrap();
    // Direction vector
    let dir_vec: Vec<(isize, isize)> = vec![(-1, 0), (1,0),(0,-1),(0,1)];
    for ele in dir_vec {
        let mut curr_score = 0;
        let mut pos = (start.0 + ele.0, start.1 + ele.1);
        while true {
            if(!map.contains_key(&pos)) {
                score *= curr_score;
                break;
            }
            else if (map.get(&pos).unwrap() >= tree_val) {
                curr_score += 1;
                score *= curr_score;
                break;
            }

            curr_score += 1;

            pos = (pos.0 + ele.0, pos.1 + ele.1);
        }
    }

    score
}

fn is_visible(start: (isize, isize), map: &HashMap<(isize, isize), usize>) -> bool {
    let tree_val = map.get(&start).unwrap();
    // Direction vector
    let dir_vec: Vec<(isize, isize)> = vec![(-1, 0), (1,0),(0,-1),(0,1)];
    for ele in dir_vec {
        let mut pos = (start.0 + ele.0, start.1 + ele.1);
        while true {
            if(!map.contains_key(&pos)) {
                return true;
            }
            else if (map.get(&pos).unwrap() >= tree_val) {
                break;
            }

            pos = (pos.0 + ele.0, pos.1 + ele.1);
        }
    }

    return false;
}

// Creates a map of Nodes from a 2D Vector of chars and returns start and end positions
fn create_map(input: &str) -> HashMap<(isize, isize), usize> {
    let char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut map: HashMap<(isize, isize), usize> = HashMap::new();
    for row in char_vec.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            map.insert((row.0 as isize, col.0 as isize), col.1.to_digit(10).unwrap() as usize);
        }
    }

    map
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let tree_grid: HashMap<(isize, isize), usize> = create_map(input);

    for ele in &tree_grid {
        if(is_visible(*ele.0, &tree_grid)) {
            sum += 1;
        }
    }

    sum
}

pub fn puzzle2(input: &str) -> i128 {
    let mut score: i128 = 0;

    let tree_grid: HashMap<(isize, isize), usize> = create_map(input);

    for ele in &tree_grid {
        let temp = scenic_score(*ele.0, &tree_grid);
        score = score.max(temp);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_08_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 21);
    }

    #[test]
    fn test_day_08_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1719);
    }

    #[test]
    fn test_day_08_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 8);
    }

    #[test]
    fn test_day_08_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 590824);
    }
}