pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    sum
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
        assert_eq!(puzzle1(EXAMPLE), 7);
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
