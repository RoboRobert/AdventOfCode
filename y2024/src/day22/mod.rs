use std::{cmp::max, collections::{HashMap, HashSet}};

trait Secrets {
    fn mix(self, other: i128) -> i128;
    fn prune(self) -> i128;
    fn new_secret(&mut self);
}

impl Secrets for i128 {
    fn mix(self, other: i128) -> i128 {
        return self^other;
    }
    fn prune(self) -> i128 {
        return self%16777216;
    }
    fn new_secret(&mut self) {
        *self = self.mix(*self<<6).prune();
        *self = self.mix(*self>>5).prune();
        *self = self.mix(*self<<11).prune();
    }
}

pub fn puzzle1(input: &str) -> i128 {
    let mut num_vec: Vec<i128> = input.lines().map(|line| line.parse::<i128>().unwrap()).collect();

    for i in 0..2000 {
        for ele in num_vec.iter_mut() {
            ele.new_secret();
        }
    }

    return num_vec.iter().sum();
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum:i128 = 0;
    let mut num_vec: Vec<i128> = input.lines().map(|line| line.parse::<i128>().unwrap()).collect();

    // Vector of vectors to store the sequence of prices and changes for each buyer
    let mut price_vec: Vec<Vec<(i128, i128)>> = vec![Vec::new();num_vec.len()];

    // Vector of hashmaps to store the prices for each sequence
    let mut price_maps: Vec<HashMap<(i128, i128, i128, i128), i128>> = vec![HashMap::new();num_vec.len()];

    let iterations: usize = 2000;
    for i in 0..iterations {
        for j in 0..num_vec.len() {
            let prev = num_vec[j]%10;
            num_vec[j].new_secret();
            let new  = num_vec[j]%10;
            price_vec[j].push((new, new-prev));
        }
    }

    for i in 0..price_vec.len() {
        for window in price_vec[i].windows(4).enumerate() {
            let sequence = (window.1[0].1, window.1[1].1,window.1[2].1,window.1[3].1);
            price_maps[i].entry(sequence).or_insert(price_vec[i][window.0+3].0);
        }
    }

    let mut sequences: HashSet<(i128,i128,i128,i128)> = HashSet::new();
    for ele in &price_maps {
        for sequence in ele {
            sequences.insert(*sequence.0);
        }
    }

    for sequence in sequences {
        let mut temp: i128 = 0;
        for ele in &price_maps {
            let val = ele.get((&sequence));
            match val {
                None => temp += 0,
                _ => temp += val.unwrap(),
            }
        }
        sum = max(sum, temp);
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_21_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 37327623);
    }

    #[test]
    fn test_day_21_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 15006633487);
    }

    #[test]
    fn test_day_21_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE2), 23);
    }

    #[test]
    fn test_day_21_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1710);
    }
}
