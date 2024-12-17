

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

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
    fn test_day_10_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 1928);
    }

    #[test]
    fn test_day_10_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6283404590840);
    }

    #[test]
    fn test_day_10_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 2858);
    }

    #[test]
    fn test_day_10_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6304576012713);
    }
}
