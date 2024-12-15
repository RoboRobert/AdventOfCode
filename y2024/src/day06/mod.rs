trait Get<T> { 
    fn get_checked(&self, index: isize) -> Option<&T>;
}

impl<T> Get<T> for Vec<T> where T : Clone {
    fn get_checked(&self, index: isize) -> Option<&T> {
        if(index < 0 || index >= self.len() as isize) {
            return None;
        }

        return self.get(index as usize);
    }
}
trait Access2D<T> {
    fn get_2d(&self, index: (isize, isize)) -> Option<&T>;
}

impl<T> Access2D<T> for Vec<Vec<T>> where T : Clone {
    fn get_2d(&self, index: (isize, isize)) -> Option<&T> {
        if(index.0 < 0 || index.1 < 0 || index.0 >= self[0].len() as isize || index.1 >= self.len() as isize) {
            return None;
        }
    
        return self[index.0 as usize].get(index.1 as usize);
    }
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut start_pos: (isize, isize) = (0,0);

    let mut char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Finds where to start
    for ele in input.lines().enumerate() {
        let found = ele.1.find('^');
        match found {
            None => continue,
            _ => {start_pos = (ele.0 as isize, found.unwrap() as isize); break;}
        }
    }

    let mut direction: (isize, isize) =  (-1,0);
    let mut pos = start_pos;
    while true {
        char_vec[pos.0 as usize][pos.1 as usize] = 'X';
        let forward = char_vec.get_2d((pos.0+direction.0, pos.1+direction.1));
        if forward == None {
            break;
        }
        if(forward.unwrap() == &'#') {
            match direction {
                // Up
                (-1,0) => {
                    direction = (0,1);
                },
                // Down
                (1,0) => {
                    direction = (0,-1);
                },
                // Left
                (0,-1) => {
                    direction = (-1,0);
                },
                // Right
                (0,1) => {
                    direction = (1,0);
                },
                _ => print!("WEIRD"),
            }
        }
        else {
            pos = (pos.0+direction.0, pos.1+direction.1);
        }
    }

    for ele in char_vec {
        for ele in ele {
            if ele == 'X' {
                sum += 1;
            }
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
    fn test_day_06_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 143);
    }

    #[test]
    fn test_day_06_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6034);
    }

    #[test]
    fn test_day_06_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 123);
    }

    #[test]
    fn test_day_06_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6305);
    }
}
