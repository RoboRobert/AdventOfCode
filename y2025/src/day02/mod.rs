pub fn puzzle1(input: &str) -> i64 {
    let mut sum: i64 = 0;
    let ranges: Vec<(i64, i64)> = input
        .split(",")
        .map(|current| {
            let mut split = current.split("-");
            return (
                split.nth(0).unwrap().parse::<i64>().unwrap(),
                split.nth(0).unwrap().parse::<i64>().unwrap(),
            );
        })
        .collect();

    for range in ranges {
        let lower = range.0;
        let upper = range.1;

        // Only look at ranges that contain numbers with an even amount of digits
        if lower.to_string().len() % 2 == 0 || upper.to_string().len() % 2 == 0 {
            for i in lower..=upper {
                let i_as_str = i.to_string();
                let split = i_as_str.split_at(i_as_str.len() / 2);
                if split.0 == split.1 {
                    sum += i;
                }
            }
        }
    }

    sum
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
    fn test_day_02_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE), 3);
    }

    #[test]
    fn test_day_02_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1052);
    }

    #[test]
    fn test_day_02_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 6);
    }

    #[test]
    fn test_day_02_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6295);
    }
}
