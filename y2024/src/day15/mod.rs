use std::collections::HashMap;
use itertools::Itertools;

// Either returns None or the nearest location of an air space
fn push(obj_vec: &HashMap<(isize, isize), char>, pos:(isize, isize), dir: (isize, isize)) -> Option<(isize,isize)> {
    // If the object is a wall, it can't be pushed
    if(obj_vec.get(&pos).unwrap() == &'#') {
        return None;
    }
    // If the object is space, it can be pushed
    else if(obj_vec.get(&pos).unwrap() == &'.') {
        return Some(pos);
    }
    // If the object is something else, return based on the objects in the direction of push
    else {
        return push(obj_vec, (pos.0+dir.0, pos.1+dir.1), dir);
    }
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let warehouse_vec: Vec<Vec<char>> = input.split("\n\n").nth(0).unwrap().lines().map(|line| line.chars().collect()).collect();
    let moves_vec: Vec<char> = input.split("\n\n").nth(1).unwrap().lines().join("").chars().collect();

    // Convert the warehouse vec into an hashmap that maps the positions to their characters
    let mut obj_map: HashMap<(isize,isize), char> = HashMap::new();

    for ele1 in warehouse_vec.iter().enumerate() {
        for ele2 in ele1.1.iter().enumerate() {
            obj_map.insert((ele1.0 as isize, ele2.0 as isize), *ele2.1);
        }
    }

    let mut pos: (isize,isize) = *obj_map.iter().find(|&map| map.1 == &'@').unwrap().0;
    for ele in moves_vec {
        let mut dir: (isize, isize) = (0,0);
        let mut result = None;
        match ele {
            '^' => dir = (-1,0),
            'v' => dir = (1,0),
            '<' => dir = (0,-1),
            '>' => dir = (0,1),
            _ => {},
        }
        result = push(&obj_map, pos, dir);

        // If the space is pushable, swap it with the value returned by push, then swap 
        // the robot with the position in the direction
        if(result.is_some()) {
            let pos1 = result.unwrap();
            let pos2 = (pos.0 + dir.0, pos.1 + dir.1);

            obj_map.insert(pos1, 'O');
            obj_map.insert(pos2, '@');

            obj_map.insert(pos, '.');
            pos = pos2;
        }
    }

    for ele in obj_map {
        if(ele.1 == 'O') {
            sum += ((100*ele.0.0) + ele.0.1) as i128;
        }
    }

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
    fn test_day_15_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 10092);
    }

    #[test]
    fn test_day_15_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1383666);
    }

    #[test]
    fn test_day_15_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 875318608908);
    }

    #[test]
    fn test_day_15_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6532);
    }
}