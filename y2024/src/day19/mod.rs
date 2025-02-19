pub fn puzzle1(input: &str) -> i128 {
    let mut sum = 0;

    // Vector of patterns
    let pattern_vec: Vec<&str> = input.split("\n\n").nth(0).unwrap().split(" ").collect();
    
    let design_vec: Vec<&str> = input.split("\n\n").nth(1).unwrap().split_whitespace().collect();

    dbg!(pattern_vec, design_vec);
    
    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum = 0;
    
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_19_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 22);
    }

    #[test]
    fn test_day_19_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 78428);
    }

    #[test]
    fn test_day_19_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 81);
    }

    #[test]
    fn test_day_19_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 0);
    }
}
