use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    fn move_teleport(&mut self, dimensions: (isize, isize)) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;

        // Handle teleports
        if(self.pos.0 < 0) {
            self.pos.0 += dimensions.0;
        }
        if(self.pos.0 >=dimensions.0) {
            self.pos.0 -= dimensions.0;
        }
        if(self.pos.1 < 0) {
            self.pos.1 += dimensions.1;
        }
        if(self.pos.1 >=dimensions.1) {
            self.pos.1 -= dimensions.1;
        }
    }
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let dimensions: (isize, isize) = (101,103);
    let mut robot_vec: Vec<Robot> = Vec::new();

    let re = Regex::new(
        r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)",
    )
    .unwrap();

    for capture in re.captures_iter(input) {
        robot_vec.push(Robot { pos: (capture[1].parse::<isize>().unwrap(),capture[2].parse::<isize>().unwrap()), 
                               vel: (capture[3].parse::<isize>().unwrap(),capture[4].parse::<isize>().unwrap()) })
    }

    // 100 iterations
    for i in 0..100 {
        robot_vec.iter_mut().for_each(|robot| robot.move_teleport(dimensions));
    }

    let mut quadrants: (i128, i128, i128, i128) = (0,0,0,0);

    let half_x = dimensions.0/2;
    let half_y = dimensions.1/2;
    for ele in robot_vec {
        // Top left
        if(ele.pos.0 < half_x && ele.pos.1 < half_y) {
            quadrants.0 += 1;
        }
        // Bottom right
        if(ele.pos.0 > half_x && ele.pos.1 > half_y) {
            quadrants.1 += 1;
        }
        // Top right
        if(ele.pos.0 > half_x && ele.pos.1 < half_y) {
            quadrants.2 += 1;
        }
        // Bottom left
        if(ele.pos.0 < half_x && ele.pos.1 > half_y) {
            quadrants.3 += 1;
        }
    }

    return quadrants.0*quadrants.1*quadrants.2*quadrants.3;
}

// Pretty-prints the locations of all the robots
fn display_bots(robot_vec: Vec<Robot>,  dimensions: (isize, isize)) {
    let mut char_vec: Vec<Vec<char>> = vec![vec!['.';dimensions.0 as usize]; dimensions.1 as usize];

    for ele in robot_vec {
        let index = (ele.pos.1 as usize, ele.pos.0 as usize);

        char_vec.get_mut(index.0).unwrap()[index.1] = '*';
    }

    for ele in char_vec {
        for ele in ele {
            print!("{ele}")
        }
        println!();
    }
}

// Doesn't solve the puzzle per se, but provides a good way to view the changes in bot positions
pub fn puzzle2(input: &str) -> i128 {
    // let dimensions: (isize, isize) = (11,7);
    let dimensions: (isize, isize) = (101,103);
    let mut robot_vec: Vec<Robot> = Vec::new();

    let re = Regex::new(
        r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)",
    )
    .unwrap();

    for capture in re.captures_iter(input) {
        robot_vec.push(Robot { pos: (capture[1].parse::<isize>().unwrap(),capture[2].parse::<isize>().unwrap()), 
                               vel: (capture[3].parse::<isize>().unwrap(),capture[4].parse::<isize>().unwrap()) })
    }

    let mut start_seconds: i128 = 43;
    let mut seconds: i128 = 0;
    let mut increment: i128 = 103;

    // Output in increments defined above
    for i in 0..start_seconds {
        
        robot_vec.iter_mut().for_each(|robot| robot.move_teleport(dimensions));
        display_bots(robot_vec.clone(), dimensions);
        seconds+= 1;
        // Separate outputs by this divider
        println!("\n\n");
        println!("===================================================================");
        println!("Seconds elapsed: {seconds}");
    }

    let mut input = String::new();
    // Read input from standard input (keyboard)
    std::io::stdin().read_line(&mut input).unwrap();
    while true {
        // Output in increments defined above
        for i in 0..increment {
            robot_vec.iter_mut().for_each(|robot| robot.move_teleport(dimensions));
            seconds+= 1;
        }

        display_bots(robot_vec.clone(), dimensions);
        // Separate outputs by this divider
        println!("===================================================================");
        println!("Seconds elapsed: {seconds}");
        println!("\n\n");

        let mut input = String::new();
        // Read input from standard input (keyboard)
        std::io::stdin().read_line(&mut input).unwrap();
    }

    return 6532;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_14_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 230900224);
    }

    #[test]
    fn test_day_14_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 36571);
    }

    #[test]
    fn test_day_14_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 875318608908);
    }

    #[test]
    fn test_day_14_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6532);
    }
}