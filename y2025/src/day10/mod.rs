#[derive(Clone, Debug)]
struct Indicators {
    lights: Vec<bool>,
}

#[derive(Clone, Debug)]
struct Button {
    toggles: Vec<usize>,
}

#[derive(Clone, Debug)]
struct Joltages {
    joltages: Vec<usize>,
}

#[derive(Clone, Debug)]
struct Machine {
    indicators: Indicators,
    toggles: Vec<Button>,
    joltages: Joltages,
}

pub fn puzzle1(input: &str) -> i64 {
    let mut sum_presses: i64 = 0;

    let machine_vec: Vec<Machine> = input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");

            let lights: Vec<bool> = split
                .nth(0)
                .unwrap()
                .trim_matches(|c| c == '(' || c == ')')
                .chars()
                .map(|character| match character {
                    '.' => return false,
                    '#' => return true,
                    _ => return false,
                })
                .collect();

            return Machine {
                indicators: Indicators { lights },
                toggles: vec![],
                joltages: Joltages { joltages: vec![] },
            };
        })
        .collect();

    dbg!(machine_vec);

    sum_presses
}

pub fn puzzle2(input: &str) -> i64 {
    let mut sum_presses: i64 = 0;

    sum_presses
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_10_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 7);
    }

    #[test]
    fn test_day_10_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 0);
    }

    #[test]
    fn test_day_10_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 0);
    }

    #[test]
    fn test_day_10_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 0);
    }
}
