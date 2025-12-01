pub fn puzzle1(input: &str) -> i128 {
    let mut horizontal: i128 = 0;
    let mut depth: i128 = 0;

    input.lines().for_each(|f| {
        let split: Vec<&str> = f.split(" ").collect();
        let (command, amount) = (split[0], split[1].parse::<i128>().unwrap());
        match command {
            "forward" => horizontal += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => panic!("Unexpected command"),
        }
    });

    return horizontal * depth;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut horizontal: i128 = 0;
    let mut depth: i128 = 0;
    let mut aim: i128 = 0;

    input.lines().for_each(|f| {
        let split: Vec<&str> = f.split(" ").collect();
        let (command, amount) = (split[0], split[1].parse::<i128>().unwrap());
        match command {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "up" => {
                aim -= amount;
            }
            "down" => {
                aim += amount;
            }
            _ => panic!("Unexpected command"),
        }
    });

    return horizontal * depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_03_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE), 150);
    }

    #[test]
    fn test_day_03_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 2272262);
    }

    #[test]
    fn test_day_03_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE), 900);
    }

    #[test]
    fn test_day_03_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 2134882034);
    }
}
