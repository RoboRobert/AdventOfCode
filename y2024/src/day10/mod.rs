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

#[derive(Debug, Clone)]
struct Move {
    pos: Option<(isize, isize)>,
    prev: i128,
}

// Does a search from the starting trailhead and returns the score
pub fn search(char_vec: &Vec<Vec<char>>, start: (isize,isize), repeats: bool) -> i128 {
    let mut score: i128 = 0;
    
    let mut moves_vec: Vec<Move> = Vec::new();
    let mut seen_vec: Vec<(isize,isize)> = Vec::new();
    
    moves_vec.push(Move{pos:Some(start),prev:-1});

    while moves_vec.len() > 0 {
        let current = moves_vec.pop().unwrap();
        match current.pos {
            None => continue,
            _ => {}
        }
        
        let pos = current.pos.unwrap();
        
        // The repeats bool decides if I allow the same 9 to be counted multiple times
        // by the same trailhead.
        // If the 9 has already been seen and no repeats, don't add it.
        if(!repeats && seen_vec.contains(&pos)) {
            continue;
        }
       
        let prev = current.prev;
        let new_prev: i128 = char_vec.get_2d(pos).unwrap().to_digit(10).unwrap() as i128;

        // If this node is not one above the previous, don't continue it.
        if(new_prev - prev != 1) {
            continue;
        }

        // If the current node is on a 9, add one to the score
        if(new_prev == 9) {
            seen_vec.push(pos);
            score += 1;
            continue;
        }
        
        // Push up, down, left, right
        moves_vec.push(Move { pos: char_vec.check_2d((pos.0-1, pos.1)), prev: new_prev });
        moves_vec.push(Move { pos: char_vec.check_2d((pos.0+1, pos.1)), prev: new_prev });
        moves_vec.push(Move { pos: char_vec.check_2d((pos.0, pos.1-1)), prev: new_prev });
        moves_vec.push(Move { pos: char_vec.check_2d((pos.0, pos.1+1)), prev: new_prev });
    }

    return score;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (i, chars) in char_vec.iter().enumerate() {
        for (j, &current_char) in chars.iter().enumerate() {
            if(current_char == '0') {
                sum += search(&char_vec, (i as isize, j as isize), false);
            }
        }
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (i, chars) in char_vec.iter().enumerate() {
        for (j, &current_char) in chars.iter().enumerate() {
            if(current_char == '0') {
                sum += search(&char_vec, (i as isize, j as isize), true);
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_10_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 36);
    }

    #[test]
    fn test_day_10_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6283404590840);
    }

    #[test]
    fn test_day_10_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 81);
    }

    #[test]
    fn test_day_10_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1366);
    }
}
