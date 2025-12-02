use std::collections::HashMap;

// Returns true or false based on the success or failure of the operation
fn do_instruction<'a>(
    instruction: (&str, &str, &str, &'a str),
    gates: &mut HashMap<&'a str, usize>,
) -> bool {
    let op1 = gates.get(instruction.0);
    let operation = instruction.1;
    let op2 = gates.get(instruction.2);
    let res_gate = instruction.3;

    if (op1 == None || op2 == None) {
        return false;
    }

    let val1 = *op1.unwrap();
    let val2 = *op2.unwrap();
    let mut res_val: usize = 0;
    match operation {
        "AND" => {
            res_val = val1 & val2;
        }
        "OR" => {
            res_val = val1 | val2;
        }
        "XOR" => {
            res_val = val1 ^ val2;
        }
        _ => {
            dbg!("WEIRD");
        }
    }

    gates.insert(res_gate, res_val);

    return true;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut gates: HashMap<&str, usize> = HashMap::new();
    let mut instructions: Vec<(&str, &str, &str, &str)> = Vec::new();

    // Parse starting state
    for ele in input.split("\n\n").nth(0).unwrap().lines() {
        let gate = ele.split(": ").nth(0).unwrap();
        let value = ele.split(": ").nth(1).unwrap().parse::<usize>().unwrap();
        gates.insert(gate, value);
    }

    // Parse instructions
    for ele in input.split("\n\n").nth(1).unwrap().lines() {
        let mut split = ele.split_whitespace();
        let instruction = (
            split.nth(0).unwrap(),
            split.nth(0).unwrap(),
            split.nth(0).unwrap(),
            split.nth(1).unwrap(),
        );
        instructions.push(instruction);
    }

    while (instructions.len() > 0) {
        let mut remove_vec: Vec<usize> = Vec::new();

        for ele in instructions.iter().enumerate() {
            if (do_instruction(*ele.1, &mut gates)) {
                remove_vec.push(ele.0);
            }
        }

        for ele in remove_vec.iter().rev() {
            instructions.remove(*ele);
        }
    }

    let mut z_vec: Vec<&str> = Vec::new();
    for ele in &gates {
        if (ele.0.chars().nth(0).unwrap() == 'z') {
            z_vec.push(ele.0);
        }
    }

    z_vec.sort();

    let mut num: i128 = 0;
    let mut exp: u32 = 0;
    for ele in z_vec {
        let val = *gates.get(&ele).unwrap() as i128;
        num += val * ((2 as i128).pow(exp));
        exp += 1;
    }

    num
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
    fn test_day_24_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 2024);
    }

    #[test]
    fn test_day_24_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 15006633487);
    }

    #[test]
    fn test_day_24_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 23);
    }

    #[test]
    fn test_day_24_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1710);
    }
}
