use std::collections::HashMap;

pub fn blink(input: &str, times: i128) -> i128 {
    let mut sum:i128 = 0;

    // Maps the current number to its worth
    let mut stones: HashMap<i128, i128> = HashMap::new();

    for ele in input.split_whitespace() {
        stones.insert(ele.parse::<i128>().unwrap(), 1);
    }

    // Blink the specified number of times
    for i in 0..times {
        let mut remove_vec: Vec<i128> = Vec::new();
        let mut add_vec: Vec<(i128,i128)> = Vec::new();
        // Loop through all stones and apply the rules
        for (key, value) in &stones {
            let num = *key;
            let worth = *value;
            remove_vec.push(num);

            let len:u32 = num.to_string().len() as u32;
            let power:i128 = (10 as i128).pow(len/2);
            // Zero case. Adds the worth of all the 0's to the 1 key, or inserts a new 1 key if doesn't exist
            if(num == 0) {
                add_vec.push((1,worth));
            }
            // Even length
            else if(len%2 == 0) {
                let num1 = key/power;
                let num2 = key%power;

                add_vec.push((num1,worth));
                add_vec.push((num2,worth));
            }
            // Multiplies by 2024
            else {
                let new_num = key*2024;
                add_vec.push((new_num,worth));
            }
        }
        for ele in remove_vec {
            stones.remove(&ele);
        }
        for ele in add_vec {
            stones.entry(ele.0).and_modify(|val| *val += ele.1).or_insert(ele.1);
        }
    }

    for ele in stones {
        sum += ele.1;
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
