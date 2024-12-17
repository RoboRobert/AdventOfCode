trait Access2D<T> {
    fn get_2d(&self, index: (isize, isize)) -> Option<&T>;
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
}

// Does a search from the starting trailhead and returns the score
pub fn search(char_vec: &Vec<&str>, start: (isize,isize), bounds: (usize, usize)) -> i128 {
    let mut num_xmases = 0;
    
    let mut moves_vec: Vec<Option<Move>> = Vec::new();
    
    moves_vec.push(Some(Move { x: start.0, y: start.1, word:"X".to_string() }));

    while moves_vec.len() > 0 {
        let current = moves_vec.pop().unwrap();
        match current {
            None => continue,
            _ => {}
        }

        let new_current = current.unwrap();
        let current_word = new_current.word;
        if(current_word == "XMAS") {
            num_xmases += 1;
        }
        let x = new_current.x;
        let y = new_current.y;
        
        moves_vec.push(get_move(char_vec, current_word.clone(), x-1, y-1, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x, y-1, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x+1, y-1, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x-1, y, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x+1, y, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x-1, y+1, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x, y+1, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x+1, y+1, bounds));
    }

    return num_xmases;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

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
    fn test_day_10_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 1928);
    }

    #[test]
    fn test_day_10_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6283404590840);
    }

    #[test]
    fn test_day_10_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 2858);
    }

    #[test]
    fn test_day_10_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6304576012713);
    }
}
