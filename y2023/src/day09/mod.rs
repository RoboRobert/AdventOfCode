pub fn puzzle1(input: &str) -> i128 {
    let mut extrapolated_vec: Vec<i128> = Vec::new();
    for current_line in input.lines() {
        //Parses each line into a vector of i128
        let current_nums_str: Vec<&str> = current_line.split_whitespace().collect();
        let mut current_nums: Vec<i128> = Vec::new();
        for num in current_nums_str {
            current_nums.push(num.parse::<i128>().unwrap());
        }

        //Then makes a vector of vectors that stores the tree of values
        let mut tree_nums: Vec<Vec<i128>> = Vec::new();
        tree_nums.push(current_nums);
        
        //Works its way down the tree
        let mut final_row = false;
        let mut level: usize = 0;
        while final_row == false {
            final_row = true;
            let mut new_vec: Vec<i128> = Vec::new();

            //Adds the differences to a new row in the tree
            for index in 0..tree_nums[level].len()-1 {
                let difference = tree_nums[level][index+1] - tree_nums[level][index];

                new_vec.push(difference);
                if difference != 0 {
                    final_row = false;
                }
            }

            tree_nums.push(new_vec);
            level += 1;
        }

        let mut current_value: i128 = 0;
        for row in (1..tree_nums.len()).rev() {
            let prev_end = tree_nums[row-1].last().unwrap();
            current_value = current_value + prev_end;
        }

        extrapolated_vec.push(current_value);
    }

    let mut sum:i128= 0;
    for number in extrapolated_vec {
        sum+= number;
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut extrapolated_vec: Vec<i128> = Vec::new();
    for current_line in input.lines() {
        //Parses each line into a vector of i128
        let current_nums_str: Vec<&str> = current_line.split_whitespace().collect();
        let mut current_nums: Vec<i128> = Vec::new();
        for num in current_nums_str {
            current_nums.push(num.parse::<i128>().unwrap());
        }

        //Then makes a vector of vectors that stores the tree of values
        let mut tree_nums: Vec<Vec<i128>> = Vec::new();
        tree_nums.push(current_nums);
        
        //Works its way down the tree
        let mut final_row = false;
        let mut level: usize = 0;
        while final_row == false {
            final_row = true;
            let mut new_vec: Vec<i128> = Vec::new();

            //Adds the differences to a new row in the tree
            for index in 0..tree_nums[level].len()-1 {
                let difference = tree_nums[level][index+1] - tree_nums[level][index];

                new_vec.push(difference);
                if difference != 0 {
                    final_row = false;
                }
            }

            tree_nums.push(new_vec);
            level += 1;
        }

        let mut current_value: i128 = 0;
        for row in (1..tree_nums.len()).rev() {
            let prev_start = tree_nums[row-1].first().unwrap();

            current_value = prev_start - current_value;
        }

        extrapolated_vec.push(current_value);
    }

    let mut sum:i128= 0;
    for number in extrapolated_vec {
        sum+= number;
    }

    return sum;
}