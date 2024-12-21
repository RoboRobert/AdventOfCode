use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Move {
    num: i128,
    from: usize,
    to: usize,
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let num_stacks = input.lines().nth(0).unwrap().len() / 4 + 1;

    let mut stack_vec: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    for ele in input.split("\n\n").nth(0).unwrap().lines() {
        let mut current_stack: usize = 0;

        let mut i = 1;
        while i < ele.len() {
            let check_char = ele.chars().nth(i).unwrap();
            if (check_char.is_alphabetic()) {
                stack_vec[current_stack].insert(0, check_char);
            }

            current_stack += 1;
            i += 4;
        }
    }

    return stack_vec;
}

fn parse_moves(input: &str) -> Vec<Move> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let move_vec = re
        .captures_iter(input)
        .map(|capture| Move {
            num: capture[1].parse::<i128>().unwrap(),
            from: capture[2].parse::<usize>().unwrap() - 1,
            to: capture[3].parse::<usize>().unwrap() - 1,
        })
        .collect();
    return move_vec;
}

fn do_puzzle(input: &str, multiple: bool) -> String {
    let mut stack_vec = parse_stacks(input);
    let mut move_vec = parse_moves(input);

    for ele in move_vec {
        let mut temp: Vec<char> = Vec::new();
        for i in 0..ele.num {
            temp.push(stack_vec[ele.from].pop().unwrap());
        }
        // Puzzle 2
        if(multiple) {
            temp.reverse();
        }
        
        stack_vec[ele.to].append(&mut temp);
    }

    let mut ret_string = String::new();

    for ele in stack_vec.iter_mut() {
        ret_string.push(ele.pop().unwrap());
    }

    return ret_string;
}

pub fn puzzle1(input: &str) -> String {
    do_puzzle(input, false)
}

pub fn puzzle2(input: &str) -> String {
    do_puzzle(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_05_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), "CMZ");
    }

    #[test]
    fn test_day_05_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "TPGVQPFDH");
    }

    #[test]
    fn test_day_05_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), "MCD");
    }

    #[test]
    fn test_day_05_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "DMRDFRHHH");
    }
}
