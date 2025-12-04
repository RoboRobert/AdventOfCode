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

pub fn puzzle2(input: &str) -> i64 {
    let sum: i64 = 0;

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
        assert_eq!(puzzle1(INPUT), 31839939622);
    }

    #[test]
    fn test_day_03_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 4174379265);
    }

    #[test]
    fn test_day_03_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 41662374059);
    }
}
