use std::collections::HashMap;

pub fn count_digits(num: i128) -> u32 {
    let mut new = num;
    if(new == 0) {
        return 1;
    }
    let mut count = 0;
    while(new > 0) {
        new = new/10;
        count += 1;
    }

    return count;
}

pub fn blink(input: &str, times: i128) -> i128 {
    let mut sum:i128 = 0;

    // Maps the current number to its worth
    let mut stones: HashMap<i128, i128> = HashMap::new();

    for ele in input.split_whitespace() {
        stones.insert(ele.parse::<i128>().unwrap(), 1);
    }

    // Blink the specified number of times
    for i in 0..times {
        // Loop through all stones and apply the rules
        for stone in stones {
            let num = stone.0;
            let worth = stone.1;
            let len:u32 = count_digits(stone.0);
            let power:i128 = (10 as i128).pow(len/2);

            stones.entry(num).and_modify(|val| *val = 0);
            // Zero case. Adds the worth of all the 0's to the 1 key, or inserts a new 1 key if doesn't exist
            if(num == 0) {
                stones.entry(1).and_modify(|val| *val += worth).or_insert(worth);
            }
            // Even length (no strings)
            else if(len%2 == 0) {
                let num1 = num/power;
                let num2 = num/power;

                stones.entry(num1).and_modify(|val| *val += worth).or_insert(worth);
                stones.entry(num2).and_modify(|val| *val += worth).or_insert(worth);
            }
            // Multiplies all by 2024
            else {
                let new_num = num*2024;
                stones.entry(new_num).and_modify(|val| *val += worth).or_insert(worth);
            }
        }
    }

    for ele in stones.values() {
        sum += ele;
    }
    return sum;
}

pub fn puzzle1(input: &str) -> i128 {
    return blink(input, 25);
}

pub fn puzzle2(input: &str) -> i128 {
    return blink(input, 75);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_11_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 55312);
    }

    #[test]
    fn test_day_11_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 203953);
    }

    #[test]
    fn test_day_11_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 81);
    }

    #[test]
    fn test_day_11_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1366);
    }
}
