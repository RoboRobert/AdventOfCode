use std::ops::Index;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Equation {
    result: i128,
    operands: Vec<i128>,
}

// Returns whether the given ops work
fn do_ops(eq: Equation, ops: String) -> bool {
    let mut value = eq.operands[0];
    for i in 1..eq.operands.len() {
        match(ops.chars().nth(i-1).unwrap()) {
            '+' => value = value + eq.operands[i],
            '*' => value = value * eq.operands[i],
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
        let base: u32 = 2;
        let num_ops:u32 = ele.operands.len() as u32 -1;

        let mut value: u16 = 0b0000000;
        for i in 0..base.pow(num_ops) {
            let s_str: String = format!("{value:016b}");
            let mut ops_str: String = "".to_string();

            let start_i = (16-num_ops) as usize;
            for i in start_i..s_str.len() {
                match(s_str.chars().nth(i).unwrap()) {
                    '0' => ops_str += "+",
                    '1' => ops_str += "*",
                    _ =>  {dbg!("WEIRD");}
                }
            }

            if(do_ops(ele.clone(), ops_str)) {
                sum += ele.result;
                break;
            }
            
            value += 1;
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
    fn test_day_07_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 143);
    }

    #[test]
    fn test_day_07_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6034);
    }

    #[test]
    fn test_day_07_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 123);
    }

    #[test]
    fn test_day_07_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6305);
    }
}
