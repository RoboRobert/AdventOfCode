pub fn puzzle1(input: &str) -> u32 {
    let mut left: Vec<u32> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect();
    let mut right: Vec<u32> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect();

    left.sort();
    right.sort();

    let mut sum: u32 = 0;

    for i in 0..left.len() {
        sum += left[i].abs_diff(right[i])
    }

    return sum;
}

pub fn puzzle2(input: &str) -> u64 {
    let mut left: Vec<u64> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<u64>()
                .unwrap()
        })
        .collect();
    let mut right: Vec<u64> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u64>()
                .unwrap()
        })
        .collect();

    left.sort();
    right.sort();

    let mut sum: u64 = 0;

    for ele in left {
        sum += (right.iter().filter(|&element| *element == ele).count() as u64) * ele;
    }

    return sum;
}
