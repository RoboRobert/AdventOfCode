pub fn puzzle1(input: &str) -> i64 {
    0
}

pub fn puzzle2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_04_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 357);
    }

    #[test]
    fn test_day_04_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 17316);
    }

    #[test]
    fn test_day_04_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 3121910778619);
    }

    #[test]
    fn test_day_04_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 171741365473332);
    }
}
