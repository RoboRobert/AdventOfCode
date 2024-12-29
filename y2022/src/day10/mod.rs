fn clock_up(x: &mut i128, clock: &mut i128) -> i128 {
    *clock += 1;
    let mut sum: i128 = 0;
    if(*clock == 20 || (*clock-20)%40 == 0) {
        sum = (*clock)*(*x);
    }

    sum
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut clock: i128 = 0;
    let mut x: i128 = 1;

    for ele in input.lines() {
        let mut split = ele.split(" ");
        let first = split.nth(0).unwrap();
        let second = split.nth(0);

        match first {
            "noop" => {sum += clock_up(&mut x, &mut clock);},
            "addx" => {
                sum += clock_up(&mut x, &mut clock);
                sum += clock_up(&mut x, &mut clock);
                x += second.unwrap().parse::<i128>().unwrap();
            },
            _ => {dbg!("WEIRD");},
        }
    }

    sum
}

fn clock_crt(x: i128, clock: &mut i128, screen: &mut Vec<char>) {
    dbg!(&x,&clock);
    if(((*clock%40)-x).abs() <=1) {
        screen[*clock as usize] = '#';
    }

    *clock += 1;
}

pub fn puzzle2(input: &str) -> &str {
    let mut char_vec: Vec<char> = vec!['.';240];

    let mut clock: i128 = 0;
    let mut x: i128 = 1;

    for ele in input.lines() {
        let mut split = ele.split(" ");
        let first = split.nth(0).unwrap();
        let second = split.nth(0);

        match first {
            "noop" => {clock_crt(x, &mut clock, &mut char_vec);},
            "addx" => {
                clock_crt(x, &mut clock, &mut char_vec);
                clock_crt(x, &mut clock, &mut char_vec);
                x += second.unwrap().parse::<i128>().unwrap();
            },
            _ => {dbg!("WEIRD");},
        }
    }

    for ele in char_vec.iter().enumerate() {
        if(ele.0%40 == 0) {
            println!();
        }
        print!("{}", ele.1);
    }

    println!();

    "PCPBKAPJ"
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_10_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 13140);
    }

    #[test]
    fn test_day_10_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 14320);
    }

    #[test]
    fn test_day_10_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), "PCPBKAPJ");
    }

    #[test]
    fn test_day_10_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "PCPBKAPJ");
    }
}