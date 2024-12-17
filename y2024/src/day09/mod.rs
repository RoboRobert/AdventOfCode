pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut converted: Vec<i128> = Vec::new();
    // Convert the existing string to include length, whitespace and file ids
    let mut current_id: i128 = 0;
    let mut file_bool = true;

    for ele in input.chars() {
        let mut repeated: i128 = -1;
        let num_repeats = ele.to_digit(10).unwrap();
        match file_bool {
            // Set file block ID
            true => {
                repeated = current_id;
                current_id += 1;
            }
            // Set whitespace to -1
            false => {
                repeated = -1;
            }
            _ => {
                dbg!("WEIRD");
            }
        }

        for i in 0..num_repeats {
            converted.push(repeated);
        }
        file_bool = !file_bool;
    }

    // Then do a bunch of swapping and add up positions times IDs
    let mut iStart: usize = 0;
    let mut iEnd: usize = converted.len() - 1;
    while (iStart < iEnd) {
        // Move iStart until it finds whitespace
        while (converted[iStart] != -1) {
            iStart += 1;
        }

        // Move iEnd until it finds non-whitespace
        while (converted[iEnd] == -1) {
            iEnd -= 1;
        }

        // Then swap
        converted.swap(iStart, iEnd);
    }

    converted.swap(iStart, iEnd);

    // This is the easiest way to do it lol
    for ele in converted.iter().enumerate() {
        if (ele.1 < &0) {
            continue;
        }
        sum += ele.0 as i128 * ele.1;
    }

    return sum;
}

#[derive(Debug,Copy,Clone)]
struct FileGroup {
    size: i128,
    id: i128,
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut converted: Vec<FileGroup> = Vec::new();
    // Convert the existing string to include length, whitespace and file ids
    let mut current_id: i128 = 0;
    let mut file_bool = true;

    for ele in input.chars() {
        let mut repeated: i128 = -1;
        let num_repeats = ele.to_digit(10).unwrap() as i128;
        match file_bool {
            // Set file block ID
            true => {
                repeated = current_id;
                current_id += 1;
            }
            // Set whitespace to -1
            false => {
                repeated = -1;
            }
            _ => {
                dbg!("WEIRD");
            }
        }

        converted.push(FileGroup {
            size: num_repeats,
            id: repeated,
        });
        file_bool = !file_bool;
    }

    let mut iStart: usize = 0;
    let mut iCurrent: usize = converted.len() - 1;
    while (iCurrent > 0) {
        if(converted[iCurrent].id == -1) {
            iCurrent -= 1;
            continue;
        }

        let size1 = converted[iCurrent].size;

        // Then check for whitespace from the start of the array
        iStart = 0;
        while (iStart < iCurrent) {
            iStart += 1;

            // If the whitespace gap is large enough, move the data around
            let new_test = converted[iStart];
            if(new_test.size >= size1 && new_test.id == -1) {
                let new_data = converted[iCurrent].clone();
                converted[iStart].size -= size1;
                converted[iCurrent].id = -1;
                converted.insert(iStart, new_data);
                break;
            }
        }

        iCurrent -= 1;
    }

    let mut id_arr: Vec<i128> = Vec::new();
    // Convert everything back to an array of ids
    for ele in converted {
        for i in 0..ele.size {
            id_arr.push(ele.id);
        }
    }

    for ele in id_arr.iter().enumerate() {
        if (ele.1 < &0) {
            continue;
        }
        sum += ele.0 as i128 * ele.1;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_09_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 1928);
    }

    #[test]
    fn test_day_09_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6283404590840);
    }

    #[test]
    fn test_day_09_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 2858);
    }

    #[test]
    fn test_day_09_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6304576012713);
    }
}
