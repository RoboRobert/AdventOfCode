trait Access2D<T> {
    fn get_2d(&self, index: (isize, isize)) -> Option<&T>;
    fn check_2d(&self, index: (isize, isize)) -> Option<(isize, isize)>;
}

impl<T> Access2D<T> for Vec<Vec<T>> {
    fn get_2d(&self, index: (isize, isize)) -> Option<&T> {
        if (index.0 < 0
            || index.1 < 0
            || index.0 >= self[0].len() as isize
            || index.1 >= self.len() as isize)
        {
            return None;
        }

        return self[index.0 as usize].get(index.1 as usize);
    }

    fn check_2d(&self, index: (isize, isize)) -> Option<(isize, isize)> {
        if (index.0 < 0
            || index.1 < 0
            || index.0 >= self[0].len() as isize
            || index.1 >= self.len() as isize)
        {
            return None;
        }

        return Some(index);
    }
}

// Flood fills and returns price
pub fn flood_fill(char_vec: &mut Vec<Vec<char>>, start:(isize,isize)) -> i128 {
    if(char_vec.get_2d(start).unwrap() == &'.') {
        return 0;
    }
    let char_type = char_vec[start.0 as usize][start.1 as usize];
    let mut area: i128 = 0;
    let mut peri: i128 = 0;

    let mut moves_vec: Vec<Option<(isize,isize)>> = Vec::new();
    let mut seen_vec: Vec<(isize,isize)> = Vec::new();
    
    moves_vec.push(Some(start));

    while moves_vec.len() > 0 {
        let current = moves_vec.pop().unwrap();
        match current {
            None => {peri += 1; continue;},
            _ => {}
        }
        
        let pos = current.unwrap();
        
        // If already seen, don't add it
        if(seen_vec.contains(&pos)) {
            continue;
        }
        
        // If the current char is of the correct type, add one to the area
        if(char_vec.get_2d(pos).unwrap() == &char_type) {
            char_vec[pos.0 as usize][pos.1 as usize] = '.';
            area += 1;

            // Push up, down, left, right
            moves_vec.push(char_vec.check_2d((pos.0-1, pos.1)));
            moves_vec.push(char_vec.check_2d((pos.0+1, pos.1)));
            moves_vec.push(char_vec.check_2d((pos.0, pos.1-1)));
            moves_vec.push(char_vec.check_2d((pos.0, pos.1+1)));

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

    let mut char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let len1 = char_vec.len();
    let len2 = char_vec[0].len();
    for i in 0..len1 {
        for j in 0..len2 {
            sum += flood_fill(&mut char_vec, (i as isize, j as isize));
        }
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum: i128 = 0;

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
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
        assert_eq!(puzzle2(EXAMPLE), 81);
    }

    #[test]
    fn test_day_12_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1366);
    }
}
