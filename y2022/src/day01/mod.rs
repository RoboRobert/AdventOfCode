fn get_calories(input: &str) -> Vec<i128> {
    let mut calorie_vec: Vec<i128> = Vec::new();

    for ele in input.split("\n\n") {
        let mut total:i128 = 0;
        for ele2 in ele.split_whitespace() {
            total += ele2.parse::<i128>().unwrap();
        }
        calorie_vec.push(total);
    }
    calorie_vec.sort();

    return calorie_vec;
}

pub fn puzzle1(input: &str) -> i128  {
    get_calories(input).pop().unwrap()
}

pub fn puzzle2(input: &str) -> i128 {
    let cal_vec = get_calories(input);
    cal_vec[cal_vec.len()-3..cal_vec.len()].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_03_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 24000);
    }

    #[test]
    fn test_day_03_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 68292);
    }

    #[test]
    fn test_day_03_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 45000);
    }

    #[test]
    fn test_day_03_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 203203);
    }
}