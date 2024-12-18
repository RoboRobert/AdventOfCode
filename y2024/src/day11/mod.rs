pub fn blink(input: &str, times: i128) -> i128 {
    let mut stones: Vec<i128> = Vec::new();

    for ele in input.split_whitespace() {
        stones.push(ele.parse::<i128>().unwrap());
    }

    // Blink the specified number of times
    for i in 0..times {
        // Loop through all stones and apply the rules
        let mut j: usize = 0;
        while(j < stones.len()) {
            let current = stones[j].to_string();
            let len = current.len();
            if(stones[j] == 0) {
                stones[j] = 1;
            }
            // Even length
            else if(len%2 == 0) {
                let new_1 = current[0..len/2].parse::<i128>().unwrap();
                let new_2 = current[len/2..len].parse::<i128>().unwrap();
                
                stones[j] = new_1;
                stones.insert(j+1, new_2);
                j += 1;
            }
            else {
                stones[j]*=2024;
            }

            j += 1;
        }
    }
    
    return stones.len() as i128;
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
