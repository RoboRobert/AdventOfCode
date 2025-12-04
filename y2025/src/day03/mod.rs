use std::convert;

pub fn puzzle1(input: &str) -> i64 {
    let mut sum: i64 = 0;

    input.lines().for_each(|line| {
        let mut biggest_joltage = 0;
        let mut biggest_battery: i64 = -1;
        let mut biggest_pos: usize = 0;

        let battery_vec: Vec<i64> = line
            .chars()
            .map(|char| char.to_digit(10).unwrap().try_into().unwrap())
            .collect();

        for (position, battery) in battery_vec.iter().enumerate() {
            if battery > &biggest_battery {
                biggest_battery = *battery;
                biggest_pos = position;
            }
        }

        for (position, battery) in battery_vec.iter().enumerate() {
            if position == biggest_pos {
                continue;
            }
            if position < biggest_pos && (battery * 10) + biggest_battery > biggest_joltage {
                biggest_joltage = (battery * 10) + biggest_battery;
                continue;
            }
            if position > biggest_pos && (biggest_battery * 10) + battery > biggest_joltage {
                biggest_joltage = (biggest_battery * 10) + battery;
                continue;
            }
        }

        sum += biggest_joltage;
    });

    sum
}

fn convert_vec(input: &Vec<(usize, i64)>) -> i64 {
    let mut result = 0;
    input.iter().for_each(|item| {
        result *= 10;
        result += item.1;
    });

    return result;
}

pub fn puzzle2(input: &str) -> i64 {
    let mut sum: i64 = 0;

    input.lines().for_each(|line| {
        let mut biggest_pos_vec: Vec<(usize, i64)> = vec![];

        let battery_vec: Vec<i64> = line
            .chars()
            .map(|char| char.to_digit(10).unwrap().try_into().unwrap())
            .collect();

        for _ in 0..12 {
            let mut temp_biggest_vec: Vec<(usize, i64)> = biggest_pos_vec.clone();

            for (position, battery) in battery_vec.iter().enumerate() {
                if !biggest_pos_vec.contains(&(position, *battery)) {
                    let mut test_biggest_vec: Vec<(usize, i64)> = biggest_pos_vec.clone();
                    test_biggest_vec.push((position, *battery));
                    test_biggest_vec.sort_by(|a, b| a.0.cmp(&b.0));
                    if convert_vec(&test_biggest_vec) > convert_vec(&temp_biggest_vec) {
                        temp_biggest_vec = test_biggest_vec.clone();
                    }
                }
            }

            biggest_pos_vec = temp_biggest_vec;
        }

        sum += convert_vec(&biggest_pos_vec);
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_03_puzzle1_example1() {
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
