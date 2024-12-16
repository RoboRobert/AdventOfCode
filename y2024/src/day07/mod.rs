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
        match (ops[i - 1]) {
            1 => value = value + eq.operands[i],
            2 => value = value * eq.operands[i],
            3 => {
                let mut temp_str = value.to_string();
                let add_str = eq.operands[i].to_string();
                temp_str += add_str.as_str();
                value = temp_str.parse::<i128>().unwrap()
            }
            _ => {
                dbg!("WEIRD");
            }
        }
    }

    if value == eq.result {
        return true;
    }

    return false;
}

fn do_puzzle(input: &str, total_ops: u32) -> i128 {
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

    let max_ops = equation_vec
        .iter()
        .max_by_key(|x| x.operands.len())
        .unwrap()
        .operands
        .len();
    // Precompute all permutations
    let mut total_perms: Vec<Vec<Vec<u32>>> = Vec::new();
    for i in 1..max_ops {
        total_perms.push(
            (1 as u32..=i as u32)
                .map(|_| 1 as u32..=total_ops)
                .multi_cartesian_product()
                .collect(),
        );
    }

    for ele in equation_vec {
        let ops_index: usize = ele.operands.len() - 2;

        // Goes through the precomputed permutations for the equation
        for perm in &total_perms[ops_index] {
            if (do_ops(ele.clone(), perm.clone())) {
                sum += ele.result;
                break;
            }
        }
    }

    return sum;
}

pub fn puzzle1(input: &str) -> i128 {
    return do_puzzle(input, 2);
}

pub fn puzzle2(input: &str) -> i128 {
    return do_puzzle(input, 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_07_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 3749);
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
        assert_eq!(puzzle2(INPUT), 275791737999003);
    }
}
