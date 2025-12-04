fn convert_vec(input: &Vec<(usize, i64)>) -> i64 {
    let mut result = 0;
    input.iter().for_each(|item| {
        result *= 10;
        result += item.1;
    });

    return result;
}

fn do_puzzle(input: &str, iterations: i64) -> i64 {
    let mut sum: i64 = 0;

    input.lines().for_each(|line| {
        let mut biggest_vec: Vec<(usize, i64)> = vec![];

        let battery_vec: Vec<i64> = line
            .chars()
            .map(|char| char.to_digit(10).unwrap().try_into().unwrap())
            .collect();

        for _ in 0..iterations {
            let mut temp_biggest_vec: Vec<(usize, i64)> = biggest_vec.clone();

            for (position, battery) in battery_vec.iter().enumerate() {
                if !biggest_vec.contains(&(position, *battery)) {
                    let mut test_biggest_vec: Vec<(usize, i64)> = biggest_vec.clone();
                    test_biggest_vec.push((position, *battery));
                    test_biggest_vec.sort_by(|a, b| a.0.cmp(&b.0));
                    if convert_vec(&test_biggest_vec) > convert_vec(&temp_biggest_vec) {
                        temp_biggest_vec = test_biggest_vec.clone();
                    }
                }
            }

            biggest_vec = temp_biggest_vec;
        }

        sum += convert_vec(&biggest_vec);
    });

    sum
}

pub fn puzzle1(input: &str) -> i64 {
    do_puzzle(input, 2)
}

pub fn puzzle2(input: &str) -> i64 {
    do_puzzle(input, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_03_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 357);
    }

    #[test]
    fn test_day_03_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 17316);
    }

    #[test]
    fn test_day_03_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 3121910778619);
    }

    #[test]
    fn test_day_03_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 171741365473332);
    }
}
