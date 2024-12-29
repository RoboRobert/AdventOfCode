use std::collections::HashSet;

fn to_sign(x: isize) -> isize {
    match x.cmp(&0) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

fn count_visited(input: &str, length: usize) -> i128 {
    let start: (isize, isize) = (0,0);
    let mut move_vec: Vec<(isize, isize)> = Vec::new();

    // Converts the input into a list of moves
    for ele in input.lines() {
        let mut split = ele.split(" ");
        let dir = split.nth(0).unwrap();
        let count = split.nth(0).unwrap().parse::<usize>().unwrap();

        let mut move_dir: (isize, isize) = (0,0);

        match dir {
            "R" => move_dir = (0,1),
            "L" => move_dir = (0,-1),
            "U" => move_dir = (-1,0),
            "D" => move_dir = (1,0),
            _ => {dbg!("WEIRD");},
        }

        for i in 0..count {
            move_vec.push(move_dir);
        }
    }

    // Reverses the move_vec so I can just pop off the end
    move_vec.reverse();

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut rope: Vec<(isize, isize)> = vec![start;length];

    while move_vec.len() > 0 {
        let curr_move = move_vec.pop().unwrap();

        rope[length-1].0 += curr_move.0;
        rope[length-1].1 += curr_move.1;

        for i in (0..length-1 as usize).rev() {
            let diff = (rope[i+1].0 - rope[i].0, rope[i+1].1 - rope[i].1);

            if(diff.0.abs() >= 2 || diff.1.abs() >=2) {
                rope[i].0 += to_sign(diff.0);
                rope[i].1 += to_sign(diff.1);
            }

            // If it's the tail, insert the new tail position
            if(i == 0) {
                visited.insert(rope[i]);
            }
        }
    }

    visited.len() as i128
}

pub fn puzzle1(input: &str) -> i128 {
    count_visited(input, 2)
}

pub fn puzzle2(input: &str) -> i128 {
    count_visited(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_09_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 13);
    }

    #[test]
    fn test_day_09_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6311);
    }

    #[test]
    fn test_day_09_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 1);
    }

    #[test]
    fn test_day_09_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 2482);
    }
}