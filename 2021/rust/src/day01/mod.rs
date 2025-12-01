use itertools::Itertools;

pub fn puzzle1(input: &str) -> i128 {
    let first_line = input.lines().nth(0).unwrap();
    let mut bit_frequencies: Vec<
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
    fn test_day_01_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE), 7);
    }

    #[test]
    fn test_day_01_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1139);
    }

    #[test]
    fn test_day_01_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE), 5);
    }

    #[test]
    fn test_day_01_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1103);
    }
}
