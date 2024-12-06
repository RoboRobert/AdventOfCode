#[derive(Debug, PartialEq)]
struct Rule {
    first: i128,
    last: i128,
}

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
    fn test_day_06_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 143);
    }

    #[test]
    fn test_day_06_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6034);
    }

    #[test]
    fn test_day_06_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 123);
    }

    #[test]
    fn test_day_06_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6305);
    }
}
