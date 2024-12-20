#[derive(Debug, Clone, Copy)]
struct Throw {
    score: i128,
    beats: i128,
    loses: i128,
}

fn get_throw(input: &str) -> Throw {
    let rock: Throw = Throw {
        score: 1,
        beats: 3,
        loses: 2,
    };
    let paper: Throw = Throw {
        score: 2,
        beats: 1,
        loses: 3,
    };
    let scissors: Throw = Throw {
        score: 3,
        beats: 2,
        loses: 1,
    };
    match input {
        "A" | "X" => return rock,
        "B" | "Y" => return paper,
        "C" | "Z" => return scissors,
        _ => println!("WACK"),
    }

    return rock;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum = 0;
    let throw_vec: Vec<(Throw, Throw)> = input
        .lines()
        .map(|line| {
            (
                get_throw(line.split_whitespace().nth(0).unwrap()),
                get_throw(line.split_whitespace().nth(1).unwrap()),
            )
        })
        .collect();

    for ele in throw_vec {
        // Win
        if (ele.1.beats == ele.0.score) {
            sum += 6;
        }
        // Draw
        if (ele.0.score == ele.1.score) {
            sum += 3;
        }

        // No gain on loss
        sum += ele.1.score;
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum = 0;
    let throw_vec: Vec<(Throw, &str)> = input
        .lines()
        .map(|line| {
            (
                get_throw(line.split_whitespace().nth(0).unwrap()),
                line.split_whitespace().nth(1).unwrap(),
            )
        })
        .collect();

    for ele in throw_vec {
        match ele.1 {
            // Lose
            "X" => {sum += ele.0.beats},
            // Draw
            "Y" => {sum += ele.0.score + 3},
            // Win
            "Z" => {sum += ele.0.loses + 6},
            _ => {}
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_02_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 15);
    }

    #[test]
    fn test_day_02_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 10624);
    }

    #[test]
    fn test_day_02_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 12);
    }

    #[test]
    fn test_day_02_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 14060);
    }
}
