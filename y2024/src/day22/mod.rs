use std::{cmp::max, collections::HashMap};

use itertools::Itertools;

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

fn find_price(vec: &Vec<(i128, i128)>, sequence: &Vec<&i128>) -> i128 {
    let mut value: i128 = 0;
    // Use the `windows` method to create sliding windows of size 4
    let pos = vec.windows(sequence.len())
        .position(|window| {
            window[0].1 == *sequence[0] && 
            window[1].1 == *sequence[1] &&
            window[2].1 == *sequence[2] &&
            window[3].1 == *sequence[3]
        });

    match pos {
        None => value = 0,
        _ => value = vec[pos.unwrap() + sequence.len()-1].0,
    }

    return value;
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

    // let mut num_vec:Vec<i128> = vec![123];

    // Vector of vectors to store the sequence of prices and changes for each buyer
    // let mut price_maps: Vec<HashMap<(i128, i128, i128, i128), i128>> = vec![HashMap::new();num_vec.len()];
    let mut price_vec: Vec<Vec<(i128, i128)>> = vec![Vec::new();num_vec.len()];

    let iterations: usize = 2000;
    for i in 0..iterations {
        for j in 0..num_vec.len() {
            let prev = num_vec[j]%10;
            num_vec[j].new_secret();
            let new  = num_vec[j]%10;
            price_vec[j].push((new, new-prev));
        }
    }

    let int_range: Vec<i128> = (-9..9).collect();
    let total_perms: Vec<Vec<&i128>> = int_range.iter().permutations(4).collect();
    dbg!(total_perms.len());

    for perm in &total_perms {
        dbg!(perm);
        let mut temp: i128 = 0;
        for ele in &price_vec {
            temp += find_price(ele, perm);
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
        assert_eq!(puzzle2(INPUT), 979014);
    }
}
