fn simulate(start_pos: i64, amount: i64, direction: char) -> (i64, i64) {
    let mut new_pos = start_pos;
    let mut num_crossings = 0;
    match direction {
        'R' => {
            for _ in 0..amount {
                new_pos += 1;
                if new_pos == 100 {
                    new_pos = 0;
                }
                if new_pos == 0 {
                    num_crossings += 1;
                }
            }
        }
        'L' => {
            for _ in 0..amount {
                new_pos -= 1;
                if new_pos < 0 {
                    new_pos = 99
                } else if new_pos == 0 {
                    num_crossings += 1;
                }
            }
        }
        _ => {}
    }

    return (new_pos, num_crossings);
}

pub fn puzzle1(input: &str) -> i64 {
    let mut num_zeroes = 0;
    let mut dial_pos = 50;

    input.lines().for_each(|line| {
        let direction = line.chars().nth(0).unwrap();
        let amount = line[1..].parse::<i64>().unwrap();
        (dial_pos, _) = simulate(dial_pos, amount, direction);

        if dial_pos == 0 || dial_pos == 100 {
            num_zeroes += 1;
        }
    });

    num_zeroes
}

pub fn puzzle2(input: &str) -> i64 {
    let mut num_zeroes = 0;
    let mut added_zeroes = 0;
    let mut dial_pos = 50;

    input.lines().for_each(|line| {
        let direction = line.chars().nth(0).unwrap();
        let amount = line[1..].parse::<i64>().unwrap();
        (dial_pos, added_zeroes) = simulate(dial_pos, amount, direction);

        num_zeroes += added_zeroes;
    });

    num_zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_03_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE), 3);
    }

    #[test]
    fn test_day_03_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1052);
    }

    #[test]
    fn test_day_03_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE), 6);
    }

    #[test]
    fn test_day_03_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6295);
    }
}
