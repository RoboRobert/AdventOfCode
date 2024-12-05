pub fn get_in_direction(char_vec: &Vec<&str>, start_pos: (usize, usize), direction: (isize, isize), bounds: (usize, usize)) -> i128 {
    let mut built_string = String::new();
    let mut curr_x: isize = start_pos.0 as isize;
    let mut curr_y: isize = start_pos.1 as isize;
    for i in 0..4 {
        if(curr_x < 0 || curr_y < 0 || curr_x >= bounds.0 as isize || curr_y >= bounds.1 as isize) {
            continue;
        }
        built_string+=char_vec[curr_y as usize].get(curr_x as usize..(curr_x+1) as usize).unwrap();

        curr_x += direction.0;
        curr_y += direction.1;
    }

    if(built_string == "XMAS") {return 1};
    return 0;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;
    let char_vec: Vec<&str> = input.lines().collect();
    let bounds = (char_vec[0].len(), char_vec.len());

    for i in 0..bounds.1 {
        for j in 0..bounds.0 {
            sum += get_in_direction(&char_vec, (i,j), (1,0), bounds);
            sum += get_in_direction(&char_vec, (i,j), (-1,0), bounds);
            sum += get_in_direction(&char_vec, (i,j), (0,1), bounds);
            sum += get_in_direction(&char_vec, (i,j), (0,-1), bounds);
            sum += get_in_direction(&char_vec, (i,j), (1,1), bounds);
            sum += get_in_direction(&char_vec, (i,j), (1,-1), bounds);
            sum += get_in_direction(&char_vec, (i,j), (-1,1), bounds);
            sum += get_in_direction(&char_vec, (i,j), (-1,-1), bounds);
        }
    }

    return sum;
}

pub fn get_checked(char_vec: &Vec<&str>, pos: (isize, isize)) -> String {
    if(pos.0 < 0 || pos.1 < 0 || pos.0 >= char_vec[0].len() as isize || pos.1 >= char_vec.len() as isize) {
        return "".to_string();
    }

    return char_vec[pos.1 as usize].get(pos.0 as usize..(pos.0+1) as usize).unwrap().to_string();
}

pub fn get_x(char_vec: &Vec<&str>, start_pos: (isize, isize)) -> i128 {
    let mut built_1 = String::new();
    let mut built_2 = String::new();

    built_1 += &get_checked(char_vec, (start_pos.0-1,start_pos.1-1));
    built_1 += &get_checked(char_vec, start_pos);
    built_1 += &get_checked(char_vec, (start_pos.0+1,start_pos.1+1));

    built_2 += &get_checked(char_vec, (start_pos.0-1,start_pos.1+1));
    built_2 += &get_checked(char_vec, start_pos);
    built_2 += &get_checked(char_vec, (start_pos.0+1,start_pos.1-1));

    if((built_1 == "MAS" || built_1 == "SAM") && (built_2 == "MAS" || built_2 == "SAM")) {return 1;}

    return 0;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let char_vec: Vec<&str> = input.lines().collect();
    let bounds = (char_vec[0].len(), char_vec.len());

    for i in 0..bounds.1 {
        for j in 0..bounds.0 {
            sum += get_x(&char_vec, (i as isize,j as isize))
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
    fn test_day_04_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 18);
    }

    #[test]
    fn test_day_04_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 2543);
    }

    #[test]
    fn test_day_04_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 9);
    }

    #[test]
    fn test_day_04_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1930);
    }
}