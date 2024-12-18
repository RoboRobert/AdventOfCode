use regex::Regex;

fn find_values(
    Ax: i128,
    Ay: i128,
    Bx: i128,
    By: i128,
    PrizeX: i128,
    PrizeY: i128,
) -> Option<(i128, i128)> {
    let j = (PrizeY * Ax - PrizeX * Ay) / ((Ax * By) - (Bx * Ay));
    let i = ((PrizeX - PrizeY) - j * (Bx - By)) / (Ax - Ay);

    if (PrizeX == i * Ax + j * Bx && PrizeY == i * Ay + j * By) {
        return Some((i, j));
    }

    return None;
}

fn do_puzzle(input: &str, offset: i128) -> i128 {
    let mut sum: i128 = 0;

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    for capture in re.captures_iter(input) {
        let A = (
            capture[1].parse::<i128>().unwrap(),
            capture[2].parse::<i128>().unwrap(),
        );
        let B = (
            capture[3].parse::<i128>().unwrap(),
            capture[4].parse::<i128>().unwrap(),
        );
        let Prize: (i128, i128) = (
            offset + capture[5].parse::<i128>().unwrap(),
            offset +capture[6].parse::<i128>().unwrap(),
        );

        let values = find_values(A.0, A.1, B.0, B.1, Prize.0, Prize.1).unwrap_or((0, 0));

        let add = values.0 * 3 + values.1;
         sum += add;
    }

    return sum;
}

pub fn puzzle1(input: &str) -> i128 {
    return do_puzzle(input, 0);
}

pub fn puzzle2(input: &str) -> i128 {
    return do_puzzle(input, 10000000000000);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_13_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 480);
    }

    #[test]
    fn test_day_13_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 36571);
    }

    #[test]
    fn test_day_13_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 875318608908);
    }

    #[test]
    fn test_day_13_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 85527711500010);
    }
}
