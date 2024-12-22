use std::collections::HashMap;

fn move_to(start: char, end: char, pad: &HashMap<char, (isize,isize)>) -> String {
    let p1 = pad.get(&start).unwrap();
    let p2 = pad.get(&end).unwrap();
    let y_diff = (p2.0 - p1.0);
    let x_diff = (p2.1 - p1.1);

    let mut char_diff: Vec<(char, isize)> = Vec::new();

    // Go up
    if y_diff < 0 {char_diff.push(('^', y_diff));}
    // Go down
    else {char_diff.push(('v', y_diff));};

    // Go left
    if x_diff < 0 {char_diff.push(('<', x_diff));}
    // Go right
    else {char_diff.push(('>', x_diff));}
    
    let mut move_str = char_diff[0].0.to_string().repeat(char_diff[0].1.abs() as usize);
    move_str += &char_diff[1].0.to_string().repeat(char_diff[1].1.abs() as usize);
    move_str += "A";

    return move_str;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;
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
    pad2.insert('^', (0,1));
    pad2.insert('A', (0,2));
    pad2.insert('<', (1,0));
    pad2.insert('v', (1,1));
    pad2.insert('>', (1,2));

    for layer_0 in input.lines() {
        let numeric = layer_0[0..layer_0.len()-1].parse::<usize>().unwrap();
        // Layer 1
        let mut prev = 'A';
        let mut layer_1 = String::from("");
        for current in layer_0.chars() {
            layer_1 += &move_to(prev, current, &pad1);
            prev = current;
        }

        // dbg!(&layer_1);
        // dbg!(&layer_1.len());

        // Layer 2
        prev = 'A';
        let mut layer_2 = String::from("");
        for current in layer_1.chars() {
            layer_2 += &move_to(prev, current, &pad2);
            prev = current;
        }

        // dbg!(&layer_2);
        // dbg!(&layer_2.len());

        // Layer 3
        prev = 'A';
        let mut layer_3 = String::from("");
        for current in layer_2.chars() {
            layer_3 += &move_to(prev, current, &pad2);
            prev = current;
        }

        dbg!(&layer_3);
        dbg!(&layer_3.len());
        sum += (layer_3.len()*numeric) as i128;
    }
    
    sum
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
