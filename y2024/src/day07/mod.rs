use std::ops::Index;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Equation {
    result: i128,
    operands: Vec<i128>,
}

// Returns whether the given ops work for part 1
fn do_ops(eq: Equation, ops: Vec<u32>) -> bool {
    let mut value = eq.operands[0];
    for i in 1..eq.operands.len() {
        match(ops[i-1]) {
            1 => value = value + eq.operands[i],
            2 => value = value * eq.operands[i],
            3 => value = value * eq.operands[i],
            _ => {dbg!("WEIRD");}
        }
    }

    if value == eq.result {
        return true;
    }

    return false;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut equation_vec: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut split = line.split([':', ' ']);
            Equation {
                result: split.nth(0).unwrap().parse::<i128>().unwrap(),
                operands: split.skip(1).map(|e| e.parse::<i128>().unwrap()).collect(),
            }
        })
        .collect();

    for ele in equation_vec {
        let num_ops:u32 = ele.operands.len() as u32 -1;
        for perm in (1 as u32..=num_ops).map(|_| 1 as u32..=2 as u32).multi_cartesian_product() {
            if(do_ops(ele.clone(), perm)) {
                sum += ele.result;
                break;
            }
        }
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut equation_vec: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut split = line.split([':', ' ']);
            Equation {
                result: split.nth(0).unwrap().parse::<i128>().unwrap(),
                operands: split.skip(1).map(|e| e.parse::<i128>().unwrap()).collect(),
            }
        })
        .collect();

    for ele in equation_vec {
        let num_ops:u32 = ele.operands.len() as u32 -1;
        for perm in (1 as u32..=num_ops).map(|_| 1 as u32..=2 as u32).multi_cartesian_product() {
            if(do_ops(ele.clone(), perm)) {
                sum += ele.result;
                break;
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_07_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 3748);
    }

    #[test]
    fn test_day_07_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1399219271639);
    }

    #[test]
    fn test_day_07_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 11387);
    }

    #[test]
    fn test_day_07_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6305);
    }
}
