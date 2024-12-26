use std::collections::HashMap;

// Flood fills and returns price
pub fn flood_fill(map: &mut HashMap<(isize, isize), char>, start:(isize,isize)) -> i128 {
    if(map.get(&start).unwrap() == &'.') {
        return 0;
    }
    let char_type = *map.get(&start).unwrap();
    let mut area: i128 = 0;
    let mut peri: i128 = 0;

    let mut moves_vec: Vec<(isize,isize)> = Vec::new();
    let mut seen_vec: Vec<(isize,isize)> = Vec::new();
    
    moves_vec.push(start);

    while moves_vec.len() > 0 {
        let pos = moves_vec.pop().unwrap();
        let current = map.get(&pos);
        match current {
            None => {peri += 1; continue;},
            _ => {}
        }
        
        // If already seen, don't add it
        if(seen_vec.contains(&pos)) {
            continue;
        }
        
        // If the current char is of the correct type, add one to the area
        if(*current.unwrap() == char_type) {
            map.insert(pos, '.');
            area += 1;

            // Push up, down, left, right
            moves_vec.push((pos.0-1, pos.1));
            moves_vec.push((pos.0+1, pos.1));
            moves_vec.push((pos.0, pos.1-1));
            moves_vec.push((pos.0, pos.1+1));

            seen_vec.push(pos);
        }
        else {
            peri += 1;
        }
    }

    // dbg!(area, peri);

    // for ele in char_vec {
    //     for ele in ele {
    //         print!("{ele}");
    //     }
    //     println!();
    // }

    return area*peri;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    let mut starts: Vec<(isize, isize)> = Vec::new();

    for ele in input.lines().enumerate() {
        for ele1 in ele.1.chars().enumerate() {
            map.insert((ele.0 as isize, ele1.0 as isize), ele1.1);
            starts.push((ele.0 as isize, ele1.0 as isize));
        }
    }

    for ele in starts {
        sum += flood_fill(&mut map, ele);
    }
            
    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    let mut starts: Vec<(isize, isize)> = Vec::new();

    for ele in input.lines().enumerate() {
        for ele1 in ele.1.chars().enumerate() {
            map.insert((ele.0 as isize, ele1.0 as isize), ele1.1);
            starts.push((ele.0 as isize, ele1.0 as isize));
        }
    }

    for ele in starts {
        sum += flood_fill(&mut map, ele);
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
    fn test_day_12_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 1930);
    }

    #[test]
    fn test_day_12_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1352976);
    }

    #[test]
    fn test_day_12_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE2), 80);
    }

    #[test]
    fn test_day_12_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1366);
    }
}
