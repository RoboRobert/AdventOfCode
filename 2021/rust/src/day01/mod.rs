use itertools::Itertools;

pub fn puzzle1(input: &str) -> i128 {
    let mut increases: i128 = 0;
    for (prev, current) in input
        .lines()
        .map(|line| line.parse::<i128>().unwrap())
        .tuple_windows()
    {
        if current > prev {
            increases += 1;
        }
    }

    return increases;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut increases: i128 = 0;
    for (a, b, c, d) in input
        .lines()
        .map(|line| line.parse::<i128>().unwrap())
        .tuple_windows()
    {
        if (b + c + d) > (a + b + c) {
            increases += 1;
        }
    }

    return increases;
}
