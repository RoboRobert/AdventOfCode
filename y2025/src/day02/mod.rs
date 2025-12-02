pub fn puzzle1(input: &str) -> i64 {
    let mut num_zeroes = 0;

    num_zeroes
}

pub fn puzzle2(input: &str) -> i64 {
    let mut num_zeroes = 0;

    num_zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_03_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE), 3);
    }

    #[test]
    fn test_day_03_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1052);
    }

    #[test]
    fn test_day_03_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE), 6);
    }

    #[test]
    fn test_day_03_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6295);
    }
}
