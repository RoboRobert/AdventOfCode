fn contained_by(range1: (i128, i128), range2: (i128, i128)) -> bool {
    if ((range1.0 <= range2.0 && range2.1 <= range1.1) || (range2.0 <= range1.0 && range1.1 <= range2.1)) {
        return true;
    }

    return false;
}

fn overlaps(range1: (i128, i128), range2: (i128, i128)) -> bool {
    if ((range2.1 <= range1.1 && range2.1 >= range1.0) || (range1.1 <= range2.1 && range1.1 >= range2.0)) {
        return true;
    }

    return false;
}

fn parse_input(input: &str) -> Vec<((i128, i128), (i128, i128))> {
    return input
    .lines()
    .map(|line| {
        let split: Vec<&str> = line.split([',', '-']).collect();
        return (
            (
                split[0].parse::<i128>().unwrap(),
                split[1].parse::<i128>().unwrap(),
            ),
            (
                split[2].parse::<i128>().unwrap(),
                split[3].parse::<i128>().unwrap(),
            ),
        );
    })
    .collect();
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum = 0;

    let range_vec = parse_input(input);

    for ele in range_vec {
        if(contained_by(ele.0, ele.1)) {
            sum += 1;
        }
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum = 0;

    let range_vec = parse_input(input);

    for ele in range_vec {
        if(overlaps(ele.0, ele.1)) {
            sum += 1;
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
    fn test_day_04_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 2);
    }

    #[test]
    fn test_day_04_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 513);
    }

    #[test]
    fn test_day_04_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 4);
    }

    #[test]
    fn test_day_04_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 878);
    }
}
