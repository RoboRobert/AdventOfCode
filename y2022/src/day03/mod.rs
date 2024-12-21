use itertools::Itertools;

// Converts characters to their priority values
fn priority(input: char) -> i128 {
    if input.is_uppercase() {
        return (input as u8 - 38) as i128;
    }

    return (input as u8 - 96) as i128;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum = 0;

    let sack_vec: Vec<(&str, &str)> = input
        .lines()
        .map(|line| {
            (
                line.split_at(line.len() / 2).0,
                line.split_at(line.len() / 2).1,
            )
        })
        .collect();

    for ele in sack_vec {
        for check_char in ele.0.chars() {
            if (ele.1.contains(check_char)) {
                sum += priority(check_char);
                break;
            }
        }
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum = 0;

    // itertools OP
    for chunk in &input.lines().chunks(3) {
        let data: Vec<&str> = chunk.collect();
        for check_char in data[0].chars() {
            if (data[1].contains(check_char) && data[2].contains(check_char)) {
                sum += priority(check_char);
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
    fn test_day_03_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 157);
    }

    #[test]
    fn test_day_03_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 7716);
    }

    #[test]
    fn test_day_03_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 70);
    }

    #[test]
    fn test_day_03_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 2973);
    }
}
