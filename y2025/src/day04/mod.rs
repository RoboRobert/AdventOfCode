use std::collections::HashMap;

pub fn puzzle1(input: &str) -> i64 {
    let mut sum = 0;
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut char_map: HashMap<(isize, isize), char> = HashMap::new();
    let lines_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for (i, char_vec) in lines_vec.iter().enumerate() {
        for (j, character) in char_vec.iter().enumerate() {
            char_map.insert((i as isize, j as isize), *character);
        }
    }

    for element in &char_map {
        if element.1 == &'.' {
            continue;
        }
        let mut num_adjacent = 0;

        for direction in &directions {
            let checked_pos = (element.0 .0 + direction.0, element.0 .1 + direction.1);
            let checked_char = char_map.get(&checked_pos);
            if checked_char.is_some() {
                if checked_char.unwrap() == &'@' {
                    num_adjacent += 1;
                }
            }
        }

        if num_adjacent < 4 {
            sum += 1;
        }
    }

    sum
}

pub fn puzzle2(input: &str) -> i64 {
    let mut sum = 0;
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut char_map: HashMap<(isize, isize), char> = HashMap::new();
    let lines_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for (i, char_vec) in lines_vec.iter().enumerate() {
        for (j, character) in char_vec.iter().enumerate() {
            char_map.insert((i as isize, j as isize), *character);
        }
    }

    let mut removable_vec: Vec<(isize, isize)> = vec![];

    loop {
        while (removable_vec.len() > 0) {
            let element = removable_vec.pop().unwrap();
            char_map.insert(element, '.');
        }

        for element in &char_map {
            if element.1 == &'.' {
                continue;
            }
            let mut num_adjacent = 0;

            for direction in &directions {
                let checked_pos = (element.0 .0 + direction.0, element.0 .1 + direction.1);
                let checked_char = char_map.get(&checked_pos);
                if checked_char.is_some() {
                    if checked_char.unwrap() == &'@' {
                        num_adjacent += 1;
                    }
                }
            }

            if num_adjacent < 4 {
                removable_vec.push(*element.0);
                sum += 1;
            }
        }

        if removable_vec.len() == 0 {
            break;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_04_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 13);
    }

    #[test]
    fn test_day_04_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1547);
    }

    #[test]
    fn test_day_04_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 43);
    }

    #[test]
    fn test_day_04_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 8948);
    }
}
