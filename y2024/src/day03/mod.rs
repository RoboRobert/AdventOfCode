use regex::{self, Captures, Regex};

pub fn puzzle1(input: &str) -> i128 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    return re.captures_iter(input).map(|capture| {capture[1].parse::<i128>().unwrap()*capture[2].parse::<i128>().unwrap()}).sum();
}

pub fn puzzle2(input: &str) -> i128 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    
    let mut doBool: bool = true;
    let mut sum: i128 = 0;
    re.captures_iter(input).for_each(|f| {
        match(&f[0]) {
            "do()" => {doBool = true;}
            "don't()" => {doBool = false;}
            _ => {
                if(doBool) {
                    sum += f[1].parse::<i128>().unwrap()*f[2].parse::<i128>().unwrap();
                }
            }
        }
    });

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_03_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE1), 161);
    }

    #[test]
    fn test_day_03_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 166630675);
    }

    #[test]
    fn test_day_03_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE2), 48);
    }

    #[test]
    fn test_day_03_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 93465710);
    }
}