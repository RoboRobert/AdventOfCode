use std::collections::{HashMap, HashSet};

pub fn puzzle1(input: &str) -> i64 {
    let mut sum: i64 = 0;

    let input_char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut manifold_map: HashMap<(isize, isize), char> = HashMap::new();

    let mut start: (isize, isize) = (0, 0);

    for (i, char_vec) in input_char_vec.iter().enumerate() {
        for (j, character) in char_vec.iter().enumerate() {
            if character == &'S' {
                start = (i as isize, j as isize);
            }
            manifold_map.insert((i as isize, j as isize), *character);
        }
    }

    let mut tracked_beams: HashSet<(isize, isize)> = HashSet::from([(start.0 + 1, start.1)]);

    loop {
        let mut new_beams: HashSet<(isize, isize)> = HashSet::new();
        for beam in &tracked_beams {
            let unwrapped_state = manifold_map.get(beam);
            if unwrapped_state.is_none() {
                continue;
            }

            let current_state = unwrapped_state.unwrap();
            if current_state == &'^' {
                sum += 1;
                new_beams.insert((beam.0, beam.1 - 1));
                new_beams.insert((beam.0, beam.1 + 1));
            }
            if current_state == &'.' {
                manifold_map.insert(*beam, '|');
                new_beams.insert((beam.0 + 1, beam.1));
            }
        }
        if new_beams.len() == 0 {
            break;
        }

        tracked_beams = new_beams.clone();
    }

    sum
}

pub fn puzzle2(input: &str) -> i64 {
    let mut sum: i64 = 0;

    let input_char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = input_char_vec.len();
    let width = input_char_vec[0].len();

    let mut manifold_map: HashMap<(isize, isize), char> = HashMap::new();

    let mut start: (isize, isize) = (0, 0);

    for (i, char_vec) in input_char_vec.iter().enumerate() {
        for (j, character) in char_vec.iter().enumerate() {
            if character == &'S' {
                start = (i as isize, j as isize);
            }
            manifold_map.insert((i as isize, j as isize), *character);
        }
    }

    let mut tracked_beams: HashSet<(isize, isize)> = HashSet::from([(start.0 + 1, start.1)]);

    loop {
        let mut new_beams: HashSet<(isize, isize)> = HashSet::new();
        for beam in &tracked_beams {
            let unwrapped_state = manifold_map.get(beam);
            if unwrapped_state.is_none() {
                continue;
            }

            let current_state = unwrapped_state.unwrap();
            if current_state == &'^' {
                sum += 1;
                new_beams.insert((beam.0, beam.1 - 1));
                new_beams.insert((beam.0, beam.1 + 1));
            }
            if current_state == &'.' {
                manifold_map.insert(*beam, '|');
                new_beams.insert((beam.0 + 1, beam.1));
            }
        }
        if new_beams.len() == 0 {
            break;
        }

        tracked_beams = new_beams.clone();

        for i in 0..height {
            for j in 0..width {
                print!("{}", manifold_map.get(&(i as isize, j as isize)).unwrap());
            }
            println!();
        }
        println!();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_07_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 21);
    }

    #[test]
    fn test_day_07_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1642);
    }

    #[test]
    fn test_day_07_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 40);
    }

    #[test]
    fn test_day_07_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 7669802156452);
    }
}
