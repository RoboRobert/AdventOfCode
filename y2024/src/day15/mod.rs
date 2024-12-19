use itertools::Itertools;

trait Access2D<T> {
    fn get_2d(&self, index: (isize, isize)) -> Option<&T>;
    fn get_2d_mut(&mut self, index: (isize, isize)) -> Option<&mut T>;
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

    fn get_2d_mut(&mut self, index: (isize, isize)) -> Option<&mut T> {
        if (index.0 < 0
            || index.1 < 0
            || index.0 >= self[0].len() as isize
            || index.1 >= self.len() as isize)
        {
            return None;
        }

        return self[index.0 as usize].get_mut(index.1 as usize);
    }
}

#[derive(Debug, PartialEq, Clone,Copy)]
enum TypeEnum {
    WALL,
    BOX,
    ROBOT,
    SPACE
}

#[derive(Debug, Clone, Copy)]
struct Object {
    type_of: TypeEnum,
}

// Either returns None or the nearest location of an air space
fn push(obj_vec: &Vec<Vec<Object>>, pos:(isize, isize), dir: (isize, isize)) -> Option<(isize,isize)> {
    // If the object is a wall, it can't be pushed
    if(obj_vec.get_2d(pos).unwrap().type_of == TypeEnum::WALL) {
        return None;
    }
    // If the object is space, it can be pushed
    else if(obj_vec.get_2d(pos).unwrap().type_of == TypeEnum::SPACE) {
        return Some(pos);
    }
    // If the object is something else, return based on the objects in the direction of push
    else {
        return push(obj_vec, (pos.0+dir.0, pos.1+dir.1), dir);
    }
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let warehouse_vec: Vec<Vec<char>> = input.split("\n\n").nth(0).unwrap().lines().map(|line| line.chars().collect()).collect();
    let moves_vec: Vec<char> = input.split("\n\n").nth(1).unwrap().lines().join("").chars().collect();

    // Convert the warehouse vec into an object vec
    let mut obj_vec: Vec<Vec<Object>> = Vec::new();
    let mut pos: (isize, isize) = (0,0);

    for ele1 in warehouse_vec.iter().enumerate() {
        let mut objs: Vec<Object> = Vec::new();
        for ele2 in ele1.1.iter().enumerate() {
            match ele2.1 {
                '#' => objs.push(Object { type_of: TypeEnum::WALL }),
                '.' => objs.push(Object { type_of: TypeEnum::SPACE }),
                '@' => {objs.push(Object { type_of: TypeEnum::ROBOT }); pos = (ele1.0 as isize, ele2.0 as isize);},
                'O' => objs.push(Object { type_of: TypeEnum::BOX }),
                _ => {},
            }
        }
        obj_vec.push(objs);
    }

    for ele in moves_vec {
        let mut dir: (isize, isize) = (0,0);
        let mut result = None;
        match ele {
            '^' => dir = (-1,0),
            'v' => dir = (1,0),
            '<' => dir = (0,-1),
            '>' => dir = (0,1),
            _ => {},
        }
        result = push(&obj_vec, pos, dir);

        // If the space is pushable, swap it with the value returned by push, then swap 
        // the robot with the position in the direction
        if(result.is_some()) {
            let mut new_vec = obj_vec.clone();
            let pos1 = (pos.0 + dir.0, pos.1 + dir.1);
            let pos2 = result.unwrap();
            let mut obj1 = obj_vec.get_2d(pos1).unwrap().type_of;
            let mut obj2 = obj_vec.get_2d(pos2).unwrap().type_of;

            new_vec.get_2d_mut(pos1).unwrap().type_of = obj2;
            new_vec.get_2d_mut(pos2).unwrap().type_of = obj1;

            obj_vec = new_vec;

            obj1 = obj_vec.get_2d(pos).unwrap().type_of;
            obj2 = obj_vec.get_2d(pos1).unwrap().type_of;

            let mut new_vec = obj_vec.clone();
            new_vec.get_2d_mut(pos1).unwrap().type_of = obj1;
            new_vec.get_2d_mut(pos).unwrap().type_of = obj2;

            obj_vec = new_vec;

            pos = pos1;
        }
    }

    for ele1 in obj_vec.iter().enumerate() {
        for ele2 in ele1.1.iter().enumerate() {
            if ele2.1.type_of == TypeEnum::BOX {
                sum += ((100*ele1.0) + ele2.0+1) as i128;
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
    fn test_day_15_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 230900224);
    }

    #[test]
    fn test_day_15_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 36571);
    }

    #[test]
    fn test_day_15_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 875318608908);
    }

    #[test]
    fn test_day_15_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6532);
    }
}