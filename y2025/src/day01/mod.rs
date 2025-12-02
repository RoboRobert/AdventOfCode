fn normalize_value(input: i64) -> (i64, i64) {
    let num_clicks: i64 = (input / 100).abs();
    if input < 0 {
        return ((100 + input) % 100, num_clicks + 1);
    }
    if input >= 100 {
        return (input % 100, num_clicks);
    }

    return (input, num_clicks);
}

pub fn puzzle1(input: &str) -> i64 {
    let mut num_zeroes = 0;
    let mut dial_pos = 50;

    input.lines().for_each(|line| {
        let direction = line.chars().nth(0).unwrap();
        let amount = line[1..].parse::<i64>().unwrap();
        match direction {
            'L' => dial_pos = normalize_value(dial_pos - amount).0,
            'R' => dial_pos = normalize_value(dial_pos + amount).0,
            _ => {}
        }

        if dial_pos == 0 {
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
        match direction {
            'L' => (dial_pos, added_zeroes) = normalize_value(dial_pos - amount),
            'R' => (dial_pos, added_zeroes) = normalize_value(dial_pos + amount),
            _ => {}
        }

        dbg!(amount);
        dbg!(dial_pos);
        dbg!(added_zeroes);

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
        assert_eq!(puzzle1(INPUT), 448);
    }

    // #[test]
    // fn test_day_03_puzzle2_example2() {
    //     assert_eq!(puzzle2(EXAMPLE), 6);
    // }

    // #[test]
    // fn test_day_03_puzzle2_input() {
    //     assert_eq!(puzzle2(INPUT), 0);
    // }
}
