trait Get<T> { 
    fn get_checked(&self, index: isize) -> Option<T>;
}

impl<T> Get<T> for Vec<T> where T : Clone {
    fn get_checked(&self, index: isize) -> Option<T> {
        if(index < 0 || index >= self.len() as isize) {
            return None;
        }

        return Some(self[index as usize].clone());
    }
}

trait Get2D<T> { 
    fn get_2d(&self, index: (isize, isize)) -> Option<T>;
}

impl<T> Get2D<T> for Vec<Vec<T>> where T : Clone {
    fn get_2d(&self, index: (isize, isize)) -> Option<T> {
        if(index.0 < 0 || index.1 < 0 || index.0 >= self[0].len() as isize || index.1 >= self.len() as isize) {
            return None;
        }
    
        return Some(self[index.0 as usize].get(index.1 as usize).unwrap().clone());
    }
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut start_pos: (isize, isize) = (0,0);

    // let char_vec: Vec<Vec<char>> = input.lines().
    // // Finds where to start
    // for ele in input.lines().enumerate() {
    //     let found = ele.1.find('^');
    //     match found {
    //         None => continue,
    //         _ => {start_pos = (ele.0 as isize, found.unwrap() as isize); break;}
    //     }
    // }

    // let direction =  (-1,0);
    // let mut current_pos = Some(start_pos);
    // while current_pos != None {
    //     let forward = str_vec.get_2d
    //     if(str_vec.get_checked(direction.0))
    //     match direction {
    //         // Up
    //         (-1,0) => {
                
    //         },
    //         // Down
    //         (1,0) => {

    //         },
    //         // Left
    //         (0,-1) => {

    //         },
    //         // Right
    //         (0,1) => {

    //         },
    //         _ => print!("WEIRD"),
    //     }
    // }

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
