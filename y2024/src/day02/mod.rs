pub fn safe(input: Vec<i128>) -> bool {
    let increasing = (input[0] < input[1]);
    let max_diff: i128 = input.windows(2).map(|window| {window[1] - window[0]}).max().unwrap();
    let min_diff: i128 = input.windows(2).map(|window| {window[1] - window[0]}).min().unwrap();

    // All fail cases
    if(std::cmp::max(min_diff.abs(), max_diff.abs()) > 3 || std::cmp::min(min_diff.abs(), max_diff.abs()) < 1
    || (increasing && min_diff < 0) || (!increasing && max_diff > 0)) {
        return false;
    }

    return true;
}

pub fn puzzle1(input: &str) -> i128 {
    let record_vec: Vec<Vec<i128>> = input.lines().map(|line| {line.split_whitespace().map(|current| {str::to_string(current).parse::<i128>().unwrap()}).collect()}).collect();

    let mut sum: i128 = 0;

    for ele in record_vec {
        if(!safe(ele)) {
            continue;
        }

        sum += 1;
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let record_vec: Vec<Vec<i128>> = input.lines().map(|line| {line.split_whitespace().map(|current| {str::to_string(current).parse::<i128>().unwrap()}).collect()}).collect();

    let mut sum: i128 = 0;

    for ele in record_vec {
        for i in 0..ele.len() {
            let mut new_ele = ele.clone();
            new_ele.remove(i);
            if (!safe(new_ele)) {
                continue;
            }

            sum += 1;
            break;
        }
    }

    return sum;
}