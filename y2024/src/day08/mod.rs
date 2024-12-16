use std::collections::HashSet;

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

#[derive(Debug, Clone, Copy)]
struct Antenna {
    frequency: char,
    position: (isize, isize),
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut antennas: Vec<Antenna> = Vec::new();
    let mut frequencies: HashSet<char> = HashSet::new();

    // Convert the input to a 2D array of chars
    let mut char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (i, chars) in char_vec.iter().enumerate() {
        for (j, &current_char) in chars.iter().enumerate() {
            if(current_char != '.') {
                antennas.push(Antenna { frequency: current_char, position: (i as isize, j as isize) });
                frequencies.insert(current_char);
            }
        }
    }

    // For every frequency of antenna
    for ele in frequencies {
        let current_antennas:Vec<Antenna> = antennas.iter().filter(|&&antenna|antenna.frequency == ele).cloned().collect();

        // For every pair of antennas of the frequency
        for i in 0..current_antennas.len()-1 {
            for j in i+1..current_antennas.len(){
                let a1 = current_antennas[i];
                let a2 = current_antennas[j];
                // Find the slope between them
                let mut slope: (isize, isize) = (a1.position.0 - a2.position.0, a1.position.1 - a2.position.1);
                let i1 = (a1.position.0 + slope.0, a1.position.1+slope.1);
                let i2 = (a2.position.0 - slope.0, a2.position.1-slope.1);
                if(char_vec.get_2d(i1) != None) {
                    char_vec[i1.0 as usize][i1.1 as usize] = '#';
                }
                if(char_vec.get_2d(i2) != None) {
                    char_vec[i2.0 as usize][i2.1 as usize] = '#';
                }
            }
        }
    }

    for ele in char_vec {
        for current in ele {
            if current == '#' {
                sum += 1;
            }
        }
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut antennas: Vec<Antenna> = Vec::new();
    let mut frequencies: HashSet<char> = HashSet::new();

    // Convert the input to a 2D array of chars
    let mut char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (i, chars) in char_vec.iter().enumerate() {
        for (j, &current_char) in chars.iter().enumerate() {
            if(current_char != '.') {
                antennas.push(Antenna { frequency: current_char, position: (i as isize, j as isize) });
                frequencies.insert(current_char);
            }
        }
    }

    // For every frequency of antenna
    for ele in frequencies {
        let current_antennas:Vec<Antenna> = antennas.iter().filter(|&&antenna|antenna.frequency == ele).cloned().collect();

        // For every pair of antennas of the frequency
        for i in 0..current_antennas.len()-1 {
            for j in i+1..current_antennas.len(){
                let a1 = current_antennas[i];
                let a2 = current_antennas[j];
                // Find the slope between them
                let mut slope: (isize, isize) = (a1.position.0 - a2.position.0, a1.position.1 - a2.position.1);

                let mut current_pos = a1.position;
                let mut current_check = char_vec.get_2d(current_pos).cloned();
                while(current_check != None) {
                    char_vec[current_pos.0 as usize][current_pos.1 as usize] = '#'; 
                    current_pos = (current_pos.0 + slope.0, current_pos.1 + slope.1);
                    current_check = char_vec.get_2d(current_pos).cloned();
                }

                current_pos = a2.position;
                current_check = char_vec.get_2d(current_pos).cloned();
                while(current_check != None) {
                    char_vec[current_pos.0 as usize][current_pos.1 as usize] = '#'; 
                    current_pos = (current_pos.0 - slope.0, current_pos.1 - slope.1);
                    current_check = char_vec.get_2d(current_pos).cloned();
                }
            }
        }
    }

    for ele in char_vec {
        for current in ele {
            if current == '#' {
                sum += 1;
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
    fn test_day_08_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 14);
    }

    #[test]
    fn test_day_08_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 336);
    }

    #[test]
    fn test_day_08_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 34);
    }

    #[test]
    fn test_day_08_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1131);
    }
}
