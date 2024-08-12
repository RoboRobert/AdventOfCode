pub fn puzzle1(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            10 * (digits.first().unwrap()) + digits.last().unwrap()
    })
    .sum()
}

const DIGITS: [(u32, [&str; 2]); 9] = [
    (1, ["1", "one"]),
    (2, ["2", "two"]),
    (3, ["3", "three"]),
    (4, ["4", "four"]),
    (5, ["5", "five"]),
    (6, ["6", "six"]),
    (7, ["7", "seven"]),
    (8, ["8", "eight"]),
    (9, ["9", "nine"]),
];

pub fn puzzle2(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut first_pos: usize = 1000;
        let mut last_pos: usize = 0;

        let mut first: u32 = 0;
        let mut last: u32 = 0;
        for ele in DIGITS {
            let new_first: usize = std::cmp::min(line.find(ele.1[0]).unwrap_or(1000), line.find(ele.1[1]).unwrap_or(1000));
            if new_first < first_pos {
                first_pos = new_first;
                first = ele.0;
            }

            let new_last: usize = std::cmp::max(line.rfind(ele.1[0]).unwrap_or(0), line.rfind(ele.1[1]).unwrap_or(0));
            if new_last > last_pos {
                last_pos = new_last;
                last = ele.0;
            }
        }
        if last == 0 { last = first }
        (10 * first) + last
    })
    .sum()
}