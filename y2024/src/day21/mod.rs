use std::collections::HashMap;

fn manhattan_distance(p1: (isize, isize), p2: (isize, isize)) -> isize {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

// fn diff(p1: (isize, isize), p2: (isize, isize)) -> String {
//     let y_diff = (p1.0 - p2.0).abs();
//     let x_diff = (p1.1 - p2.1).abs();

//     let s1 = '^'.to_string().repeat(y_diff as usize);
//     s1 += '<'

//     return 
// }

pub fn puzzle1(input: &str) -> i128 {
    let mut pad1: HashMap<char, (isize, isize)> = HashMap::new();
    pad1.insert('7', (0,0));
    pad1.insert('8', (0,1));
    pad1.insert('9', (0,2));
    pad1.insert('4', (1,0));
    pad1.insert('5', (1,1));
    pad1.insert('6', (1,2));
    pad1.insert('1', (2,0));
    pad1.insert('2', (2,1));
    pad1.insert('3', (2,2));
    pad1.insert('0', (3,1));
    pad1.insert('A', (3,2));

    let mut pad2: HashMap<char, (isize, isize)> = HashMap::new();
    pad2.insert('7', (0,0));


    
    for ele in input.lines() {
        let mut dist: isize = 0;
        let mut prev = 'A';
        for current in ele.chars() {
            let p1 = *pad1.get(&prev).unwrap();
            let p2 = *pad1.get(&current).unwrap();
            dist += manhattan_distance(p1, p2);
            prev = current;
        }

        dbg!(dist);
    }
    
    0
}

pub fn puzzle2(input: &str) -> i128 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_21_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 126384);
    }

    #[test]
    fn test_day_21_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1372);
    }

    #[test]
    fn test_day_21_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 0);
    }

    #[test]
    fn test_day_21_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 979014);
    }
}
