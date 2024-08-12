pub fn puzzle1(input: &str) -> i32 {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let mut index_sum = 0;
    
    let mut possible: bool = true;
    for word in input.lines() {
        possible = true;
        let line_vec: Vec<&str> = word.split([',', ':', ';']).collect();

        let index = line_vec[0].split(' ').last().unwrap().parse::<i32>().unwrap();

        for line_index in 1..line_vec.len() {
            let color = line_vec[line_index].split(' ').nth(2).unwrap();
            let current_cubes = line_vec[line_index].split(' ').nth(1).unwrap().parse::<i32>().unwrap();
            if color == "blue" && current_cubes > blue_cubes || color == "green" && current_cubes > green_cubes || color == "red" && current_cubes > red_cubes {
                possible = false;
            }
        }

        if possible == true {
            index_sum += index;
        }
    }

    return index_sum;
}

pub fn puzzle2(input: &str) -> i32 {
    let mut power_sum = 0;
    
    for word in input.lines() {
        let line_vec: Vec<&str> = word.split([',', ':', ';']).collect();

        let mut fewest_blue = 0;
        let mut fewest_green = 0;
        let mut fewest_red = 0;
        for line_index in 1..line_vec.len() {
            let color = line_vec[line_index].split(' ').nth(2).unwrap();
            let current_cubes = line_vec[line_index].split(' ').nth(1).unwrap().parse::<i32>().unwrap();
            match color {
                "blue" => {
                    if current_cubes > fewest_blue {
                        fewest_blue = current_cubes;
                    }
                }
                "red" => {
                    if current_cubes > fewest_red {
                        fewest_red = current_cubes;
                    }
                }
                "green" => {
                    if current_cubes > fewest_green {
                        fewest_green = current_cubes;
                    }
                }
                &_ => {println!("Weird!");}
            }
        }

        let current_power = fewest_blue*fewest_green*fewest_red;

        power_sum += current_power;
    }

    return power_sum;
}