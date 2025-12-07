use std::collections::HashMap;

fn do_puzzle(input: &str) -> (i64, i64) {
    let mut possible_paths: i64 = 0;
    let mut num_splits: i64 = 0;

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

    let mut tracked_beams: HashMap<(isize, isize), i64> = HashMap::new();

    tracked_beams.insert((start.0 + 1, start.1), 1);

    loop {
        let mut new_beams: HashMap<(isize, isize), i64> = HashMap::new();
        for beam_element in &tracked_beams {
            let beam = beam_element.0;
            let current_value = beam_element.1;
            let unwrapped_state = manifold_map.get(beam);
            if unwrapped_state.is_none() {
                continue;
            }

            let current_state = unwrapped_state.unwrap();
            if current_state == &'^' {
                num_splits += 1;
                new_beams
                    .entry((beam.0 + 1, beam.1 - 1))
                    .and_modify(|value| *value += current_value)
                    .or_insert(*current_value);
                new_beams
                    .entry((beam.0 + 1, beam.1 + 1))
                    .and_modify(|value| *value += current_value)
                    .or_insert(*current_value);
            } else if current_state == &'.' {
                manifold_map.insert(*beam, '|');
                new_beams
                    .entry((beam.0 + 1, beam.1))
                    .and_modify(|value| *value += current_value)
                    .or_insert(*current_value);
            }
        }
        if new_beams.len() == 0 {
            break;
        }

        tracked_beams = new_beams.clone();
    }

    for beam in tracked_beams {
        possible_paths += beam.1;
    }

    (num_splits, possible_paths)
}

pub fn puzzle1(input: &str) -> i64 {
    do_puzzle(input).0
}

pub fn puzzle2(input: &str) -> i64 {
    do_puzzle(input).1
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
        assert_eq!(puzzle2(INPUT), 47274292756692);
    }
}
